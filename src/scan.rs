use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
struct Report {
    #[serde(rename = "SPApplicationsDataType")]
    apps: Vec<App>,
}

#[derive(Deserialize)]
pub struct App {
    pub _name: String,
    pub version: Option<String>,
    pub path: Option<String>,
    pub obtained_from: Option<String>,
}

pub fn scanApps() -> Vec<App> {
    let output = Command::new("system_profiler")
        .args(["SPApplicationsDataType", "-json"])
        .output()
        .expect("failed at runnin sysprofiler");

    if output.status.success() {
        let report: Report = serde_json::from_slice(&output.stdout).expect("Bad JSON");
        let mut neededVec = Vec::<App>::new();

        for x in report.apps {
            let y = x.path.as_deref();
            let z = y.unwrap_or("").split('/');
            let mut count: u64 = 0;
            for components in z {
                if components == "Applications" {
                    count += 1;
                }
            }
            if count == 1 {
                neededVec.push(x);
            }
        }
        neededVec.sort_by_key(|app| app._name.to_lowercase());
        neededVec
    } else {
        eprint!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Vec::new()
    }
}

#[derive(Deserialize)]
struct ReportBrew {
    #[serde(rename = "formulae")]
    formulas: Vec<Formulas>,
    casks: Vec<Casks>,
}

#[derive(Deserialize)]
pub struct Formulas {
    pub name: String,
    pub desc: Option<String>,
    pub homepage: Option<String>,
    pub installed: Vec<Installed>,
}

#[derive(Deserialize)]
pub struct Installed {
    pub version: Option<String>,
    pub time: Option<i64>,
    pub installed_on_request: bool,
}

#[derive(Deserialize)]
pub struct Casks {
    pub token: String,
    pub name: Vec<String>,
    pub desc: Option<String>,
    pub homepage: Option<String>,
    pub installed: Option<String>,
    pub installed_time: Option<i64>,
}

pub fn scanFormulaes() -> Vec<Formulas> {
    let output = Command::new("brew")
        .args(["info", "--json=v2", "--installed"])
        .output()
        .expect("scanFormulaes for Brew failed");
    if output.status.success() {
        let mut report: ReportBrew = serde_json::from_slice(&output.stdout).expect("Bad JSON");
        let mut neededFormulas = Vec::<Formulas>::new();
        //i hate you rust ownership
        for formula in report.formulas {
            if formula.installed.iter().any(|x| x.installed_on_request) {
                neededFormulas.push(formula);
            }
        }
        neededFormulas.sort_by_key(|formula| formula.name.to_lowercase());
        neededFormulas
    } else {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Vec::<Formulas>::new()
    }
}

pub fn scanCasks() -> Vec<Casks> {
    let output = Command::new("brew")
        .args(["info", "--json=v2", "--installed"])
        .output()
        .expect("scanCasks for Brew failed");
    if output.status.success() {
        let mut report: ReportBrew = serde_json::from_slice(&output.stdout).expect("Bad JSON");
        report.casks
    } else {
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Vec::<Casks>::new()
    }
}

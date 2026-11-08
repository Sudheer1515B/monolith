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
        let mut neededvec = Vec::<App>::new();

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
                neededvec.push(x);
            }
        }
        neededvec.sort_by_key(|app| app._name.to_lowercase());
        neededvec
    } else {
        eprint!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Vec::new()
    }
}

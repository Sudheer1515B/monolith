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
        report.apps
    } else {
        eprint!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Vec::new()
    }
}

use std::process::Command;
//prototype to see if i can pass through shell commands from rust into the system to get some output
pub fn main() {
    let output = Command::new("system_profiler")
        .args(["SPApplicationsDataType", "-json"])
        .output()
        .expect("failed at runnin sysprofiler");

    if output.status.success() {
        println!("data: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    //here output is stored as a 8 bit value, so we call String::from_utf8_lossy to convert the
    // unsigned 8 bit values in the ouput vec into a string
}

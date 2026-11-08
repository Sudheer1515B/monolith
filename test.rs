use std::process::Command;

let output = if cfg!(target_os = "windows") {
    Command::new("cmd")
        .args(["/C", "echo hello"])
        .output()
        .expect("process should execute successfully")
} else {
    Command::new("sh")
        .arg("echo hello")
        .output()
        .expect("process should execute successfully")
};

let hello = output.stdout;

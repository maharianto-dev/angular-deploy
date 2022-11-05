use std::process::Command;

pub fn run_ng_command(my_command: &str) {
    let mut cmd;
    if cfg!(target_os = "windows") {
        cmd = Command::new(format!("cmd"));
        cmd.arg("/C");
    } else {
        cmd = Command::new(format!("sh"));
        cmd.arg("-c");
    };
    cmd.arg(my_command);
    let child = cmd.spawn().expect("Failed executing command!");

    child
        .wait_with_output()
        .expect("Failed waiting child process to finish!");
}

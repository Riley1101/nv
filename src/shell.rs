use std::process::Command;

pub fn execute_command(command: &String) {
    Command::new("/usr/bin/sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
}

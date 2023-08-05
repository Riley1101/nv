use std::env;
use std::process::Command;

pub fn execute_command(p: &String) -> Result<(), ()> {
    let _ = env::set_current_dir(&p).is_ok();

    let cmd = Command::new("/usr/bin/sh")
        .arg("-c")
        .arg(format!("cd {} && nvim .", p))
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
    if cmd.success() {
        println!("Editor exited successfully");
        // quit the run_app
        return Ok(());
    } else {
        println!("Editor exited with error");
        return Err(());
    }
}

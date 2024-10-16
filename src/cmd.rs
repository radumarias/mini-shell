use std::path::Path;
use std::{env, io};

/// Read command and args.
pub fn read_cmd() -> (usize, String) {
    let mut input = String::new();
    let c = io::stdin().read_line(&mut input).unwrap();
    (c, input)
}

/// Execute a command with args.
pub fn exec_cmd(cmd: &str, args: Vec<String>) -> io::Result<(i32, String, String)> {
    if cmd == "cd" {
        let new_dir = Path::new(args.first().ok_or(io::Error::other("empty args"))?.as_str());
        if !new_dir.exists() {
            return Ok((
                1,
                "".to_string(),
                format!(
                    "cd: no such file or directory: {}",
                    new_dir.to_str().unwrap()
                ),
            ));
        }
        // set current dir
        env::set_current_dir(new_dir)?;
        return Ok((0, "".to_string(), "".to_string()));
    }

    let mut command = std::process::Command::new(cmd);
    for arg in args {
        command.arg(arg);
    }
    let output = command.output();
    match output {
        Ok(output) => {
            let exit_code = output.status.code().unwrap_or(1);
            match exit_code {
                0 => {
                    let output = String::from_utf8_lossy(&output.stdout);
                    Ok((exit_code, output.to_string(), "".to_string()))
                }
                _ => {
                    let out = String::from_utf8_lossy(&output.stdout);
                    let err = String::from_utf8_lossy(&output.stderr);
                    Ok((exit_code, out.to_string(), err.to_string()))
                }
            }
        }
        Err(err) => Err(err),
    }
}

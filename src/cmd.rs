use std::path::Path;
use std::{env, io};

use crate::print::print_err_str;

/// Read command and args.
pub fn read_cmd() -> (usize, String) {
    let mut input = String::new();
    let c = io::stdin().read_line(&mut input).unwrap();
    (c, input)
}

/// Parse a cmd line in cmd and args.
pub fn parse_cmd(input: &str) -> io::Result<(String, Vec<String>)> {
    let mut parts = input.split_whitespace();

    // parse cmd
    let cmd = match parts.next() {
        Some(cmd) => cmd,
        None => return Err(io::Error::other("empty input")),
    };

    // parse args
    let args = parts.next();
    return match args {
        Some(c) => {
            // we have args
            match c.chars().next() {
                // remove single quotes and double quotes
                Some('\'') | Some('"') => {
                    let first_quote = c.chars().next().unwrap();
                    let last_quote = c.chars().last().unwrap();
                    // check for closing quote
                    if first_quote != last_quote {
                        return Err(io::Error::other("missing closing quote"));
                    }
                    let args = c.replace(first_quote, "");
                    Ok((cmd.to_string(), vec![args]))
                }
                // no quotes
                _ => {
                    let args = c.split_whitespace().map(|s| s.to_string()).collect();
                    Ok((cmd.to_string(), args))
                }
            }
        }
        // no args
        None => Ok((cmd.to_string(), Vec::new())),
    };
}

/// Execute a command with args.
pub fn exec_cmd(cmd: &str, args: Vec<String>) -> io::Result<(i32, String)> {
    if cmd == "cd" {
        let new_dir = Path::new(args.first().ok_or(io::Error::other("empty args"))?.as_str());
        if !new_dir.exists() {
            print_err_str(
                format!(
                    "cd: no such file or directory: {}",
                    new_dir.to_str().unwrap()
                )
                .as_str(),
            );
            return Ok((1, "".to_string()));
        }
        // set current dir
        if let Err(e) = env::set_current_dir(new_dir) {
            print_err_str(format!("Error changing directory: {}", e).as_str());
        }
        return Ok((0, "".to_string()));
    }

    let mut command = std::process::Command::new(cmd);
    for arg in args {
        command.arg(arg);
    }
    let output = command.output();
    match output {
        Ok(output) => {
            let exit_code = output.status.code().unwrap_or(1);
            let output = String::from_utf8_lossy(&output.stdout);
            Ok((exit_code, output.to_string()))
        }
        Err(err) => Err(err),
    }
}

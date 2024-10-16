mod cmd;
mod print;

const PROMPT: &str = "$ ";

fn main() {
    loop {
        // print prompt
        eprint!("{PROMPT}");

        // read line
        let (c, input) = cmd::read_cmd();
        if c == 0 {
            std::process::exit(0)
        }

        // parse command and args
        match cmd::parse_cmd(input.as_str()) {
            // parse ok
            Ok((cmd, args)) => {
                match cmd.as_str() {
                    // process exit
                    "exit" | "quit" | "bye" => std::process::exit(0),
                    _ => {
                        // execute command
                        match cmd::exec_cmd(cmd.as_str(), args) {
                            // ok
                            Ok((status, output)) => {
                                match status {
                                    // success
                                    0 => print!("{output}"),
                                    // exit code
                                    _ => print::print_err_str(
                                        format!("Exit code: {status}: {output}").as_str(),
                                    ),
                                }
                            }
                            // failed
                            Err(err) => print::print_err(err),
                        }
                    }
                }
            }
            // parse failed
            Err(err) => print::print_err(err),
        }
    }
}

mod cmd;
mod parse;
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
        match parse::parse_cmd(input.as_str()) {
            // parse ok
            Ok((cmd, args)) => {
                match cmd.as_str() {
                    // process exit
                    "exit" | "quit" | "bye" => std::process::exit(0),
                    _ => {
                        // execute command
                        match cmd::exec_cmd(cmd.as_str(), args) {
                            // ok
                            Ok((status, out, _err)) => {
                                match status {
                                    // success
                                    0 => print!("{out}"),
                                    // exit code
                                    _ => {
                                        // todo: maybe we should print err also

                                        print::print_err_str(
                                            format!("error: command exited with code {status}")
                                                .as_str(),
                                        )
                                    }
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

use std::io;

/// Parse a cmd line in cmd and args.
pub fn parse_cmd(input: &str) -> io::Result<(String, Vec<String>)> {
    let parts = split(input)?;
    let parts2 = parts.clone();
    let mut iter = parts.into_iter();

    // parse cmd
    let cmd = match iter.next() {
        Some(cmd) => cmd,
        None => return Err(io::Error::other("empty input")),
    };

    // parse args
    let args = iter.next();
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
                    let args: Vec<String> =
                        parts2.into_iter().skip(1).map(|s| s.to_string()).collect();
                    Ok((cmd.to_string(), args))
                }
            }
        }
        // no args
        None => Ok((cmd.to_string(), Vec::new())),
    };
}

/// Split keeping items inside quotes in one item.
fn split(args: &str) -> io::Result<Vec<String>> {
    // todo: split iteratively by each space and using quota as boundary.
    // like that we can split commands like `/bin/sh -c 'exit 7'`
    let mut split = args.trim().splitn(2, ' ');
    let cmd = split.next();
    let args = split.next();
    let cmd = cmd
        .map(|s| s.trim())
        .ok_or(io::Error::other("empty input"))?
        .to_string();
    match args {
        Some(args) => {
            let first = args.chars().next();
            let last = args.chars().last();
            match first {
                Some('\'') | Some('"') => {
                    if first == last {
                        let args = &args[1..(args.len() - 1)];
                        Ok(vec![cmd, args.to_string()])
                    } else {
                        Err(io::Error::other("missing closing quote"))
                    }
                }
                _ => {
                    let mut args: Vec<String> =
                        args.split_whitespace().map(|s| s.to_string()).collect();
                    args.insert(0, cmd);
                    Ok(args)
                }
            }
        }
        None => Ok(vec![cmd.to_string()]),
    }
}

use std::io;

/// Parse a cmd line in cmd and args.
pub fn parse_cmd(input: &str) -> io::Result<(String, Vec<String>)> {
    let parts = split(input)?;
    let mut iter = parts.into_iter();

    // cmd
    let cmd = match iter.next() {
        Some(cmd) => cmd,
        None => return Err(io::Error::other("empty input")),
    };

    // args
    let args: Vec<String> = iter.collect();

    Ok((cmd, args))
}

/// Split cmd and args preserving items inside quotes.
fn split(line: &str) -> io::Result<Vec<String>> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut last_quote = None;
    for c in line.chars() {
        if c == ' ' && last_quote.is_none() {
            current = current.trim().to_string();
            if !current.is_empty() {
                result.push(current);
                current = String::new();
            }
        } else if (c == '"' || c == '\'')
            && (last_quote.is_none() || *last_quote.as_ref().unwrap() == c)
        {
            last_quote = if last_quote.is_none() { Some(c) } else { None };
        } else {
            current.push(c);
        }
    }
    if last_quote.is_some() {
        return Err(io::Error::other(format!(
            "unclosed quote `{}`",
            last_quote.as_ref().unwrap()
        )));
    }
    current = current.trim().to_string();
    if !current.is_empty() {
        result.push(current);
    }
    if result.is_empty() {
        return Err(io::Error::other("empty input"));
    }
    Ok(result)
}

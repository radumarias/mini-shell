use std::io;

/// Parse a cmd line in cmd and args.
pub fn parse_cmd(input: &str) -> io::Result<(String, Vec<String>)> {
    let parts = split(input)?;
    if parts.is_empty() {
        return Err(io::Error::other("empty input"));
    }
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
    Ok(result)
}

/// Any POSIX shell details not specified in this specification should be omitted (e.g. signal handling, pipes, redirection, environment variable expansion, line editing, history, etc.)
fn normalize(input: String) -> String {
    input
        .replace(['|', '>', '<', '&', ';', '$'], "")
        .replace("&&", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split("").unwrap(), Vec::<String>::new());
        assert_eq!(split("a").unwrap(), vec!["a".to_string()]);
        assert_eq!(
            split("a b").unwrap(),
            vec!["a".to_string(), "b".to_string()]
        );
        assert_eq!(
            split("a \"b\" c").unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        assert_eq!(
            split("a 'b' c").unwrap(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
        assert_eq!(
            split("a \"b c\"").unwrap(),
            vec!["a".to_string(), "b c".to_string()]
        );
        assert_eq!(
            split("a 'b c'").unwrap(),
            vec!["a".to_string(), "b c".to_string()]
        );
        assert!(split("a \"b c").is_err());
        assert!(split("a 'b c").is_err());
    }

    #[test]
    fn test_parse_cmd() {
        assert!(parse_cmd("").is_err());
        assert_eq!(parse_cmd("a").unwrap(), ("a".to_string(), vec![]));
        assert_eq!(
            parse_cmd("a b").unwrap(),
            ("a".to_string(), vec!["b".to_string()])
        );
        assert_eq!(
            parse_cmd("a \"b\" c").unwrap(),
            ("a".to_string(), vec!["b".to_string(), "c".to_string()])
        );
        assert_eq!(
            parse_cmd("a 'b' c").unwrap(),
            ("a".to_string(), vec!["b".to_string(), "c".to_string()])
        );
        assert_eq!(
            parse_cmd("a \"b c\"").unwrap(),
            ("a".to_string(), vec!["b c".to_string()])
        );
        assert_eq!(
            parse_cmd("a 'b c'").unwrap(),
            ("a".to_string(), vec!["b c".to_string()])
        );
        assert!(parse_cmd("a 'b c").is_err());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("a b c".to_string()), "a b c".to_string());
        assert_eq!(normalize("a && b".to_string()), "a  b".to_string());
        assert_eq!(normalize("a | b".to_string()), "a  b".to_string());
        assert_eq!(normalize("a > b".to_string()), "a  b".to_string());
        assert_eq!(normalize("a < b".to_string()), "a  b".to_string());
        assert_eq!(normalize("a & b".to_string()), "a  b".to_string());
        assert_eq!(normalize("a ; b".to_string()), "a  b".to_string());
    }
}

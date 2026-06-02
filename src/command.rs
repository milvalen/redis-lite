#[derive(Debug, PartialEq)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    Unknown(String),
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.trim().splitn(3, ' ').collect();

        match parts.as_slice() {
            // TODO: match on "SET", "GET", "DEL" (case-insensitive)
            // "SET foo bar" → Set { key: "foo", value: "bar" }
            // "GET foo"     → Get { key: "foo" }
            // "DEL foo"     → Del { key: "foo" }
            _ => Command::Unknown(input.trim().to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_set() {
        // TODO: uncomment once parse() is implemented
        // assert_eq!(
        //     Command::parse("SET foo bar"),
        //     Command::Set { key: "foo".into(), value: "bar".into() }
        // );
    }

    #[test]
    fn test_parse_get() {
        // assert_eq!(
        //     Command::parse("GET foo"),
        //     Command::Get { key: "foo".into() }
        // );
    }

    #[test]
    fn test_parse_del() {
        // assert_eq!(
        //     Command::parse("DEL foo"),
        //     Command::Del { key: "foo".into() }
        // );
    }

    #[test]
    fn test_parse_case_insensitive() {
        // assert_eq!(
        //     Command::parse("set FOO bar"),
        //     Command::Set { key: "FOO".into(), value: "bar".into() }
        // );
    }

    #[test]
    fn test_parse_unknown() {
        assert_eq!(Command::parse("FLUSHALL"), Command::Unknown("FLUSHALL".into()));
    }
}

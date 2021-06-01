use std::fmt;

const NUMERIC_VALUE_MSG: &str = "Numeric values are not allowed.\n";
const EMPTY_VALUE_MSG: &str = "Empty values are not allowed.\n";

pub struct ParseError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error parsing message: {}\nCause: {}",
            self.message, self.cause
        )
    }
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl PartialEq<ParseError> for ParseError {
    fn eq(&self, other: &ParseError) -> bool {
        self.cause == other.cause && self.message == other.message
    }
}

impl Eq for ParseError {}

impl ParseError {
    pub fn numeric_value(msg: &str) -> Self {
        ParseError {
            message: String::from(msg),
            cause: String::from(NUMERIC_VALUE_MSG),
        }
    }

    pub fn empty_value(msg: &str) -> Self {
        ParseError {
            message: String::from(msg),
            cause: String::from(EMPTY_VALUE_MSG),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_equality() {
        let error_aa = ParseError {
            message: String::from("Test message A"),
            cause: String::from("Cause A"),
        };
        let error_bb = ParseError {
            message: String::from("Test message B"),
            cause: String::from("Cause B"),
        };
        let error_ab = ParseError {
            message: String::from("Test message A"),
            cause: String::from("Cause B"),
        };
        let error_ba = ParseError {
            message: String::from("Test message B"),
            cause: String::from("Cause A"),
        };

        assert_eq!(error_aa, error_aa);
        assert_eq!(error_bb, error_bb);

        assert_ne!(error_aa, error_bb);
        assert_ne!(error_aa, error_ba);
        assert_ne!(error_bb, error_ab);
        assert_ne!(error_bb, error_ba);
        assert_ne!(error_ab, error_ba);
    }

    #[test]
    fn test_numeric_value_error() {
        let error = ParseError::numeric_value("A numeric value");

        assert_eq!(error.message, "A numeric value");
        assert_eq!(error.cause, NUMERIC_VALUE_MSG);
    }

    #[test]
    fn test_empty_value_error() {
        let error = ParseError::empty_value("An empty value");

        assert_eq!(error.message, "An empty value");
        assert_eq!(error.cause, EMPTY_VALUE_MSG);
    }
}

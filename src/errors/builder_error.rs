use std::fmt;

const COMMAND_NOT_FOUND_MSG: &str = "Unable to find the command entered.";

pub struct BuilderError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error running command: {}\nCause: {}",
            self.message, self.cause
        )
    }
}

impl fmt::Debug for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl PartialEq<BuilderError> for BuilderError {
    fn eq(&self, other: &BuilderError) -> bool {
        return self.cause == other.cause && self.message == other.message;
    }
}

impl Eq for BuilderError {}

impl BuilderError {
    pub fn not_found(msg: &str) -> Self {
        BuilderError {
            message: String::from(msg),
            cause: String::from(COMMAND_NOT_FOUND_MSG),
        }
    }
}
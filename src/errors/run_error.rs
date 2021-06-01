use std::fmt;

//const UNKNOWN_ERROR_MSG: &str = "Unknown error, could not process command.\n";

pub struct RunError {
    pub message: String,
    pub cause: String,
}

impl fmt::Display for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error running command: {}\nCause: {}",
            self.message, self.cause
        )
    }
}

impl fmt::Debug for RunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

impl PartialEq<RunError> for RunError {
    fn eq(&self, other: &RunError) -> bool {
        return self.cause == other.cause && self.message == other.message;
    }
}

impl Eq for RunError {}

/*
impl RunError {
    pub fn unknown_error(msg: &str) -> Self {
        RunError {
            message: String::from(msg),
            cause: String::from(UNKNOWN_ERROR_MSG),
        }
    }
}
 */
//! Has the responsibility of validating the user input and fetching the apropiate command for it.
use crate::errors::parse_error::ParseError;
use crate::errors::run_error::RunError;

/// The ParsedMessage struct stores the correctly parsed input of the user.
pub struct ParsedMessage {
    /// The command entered.
    pub command: String,
    /// The remaining words entered by the user.
    pub arguments: Vec<String>,
}

impl ParsedMessage {

    /// Validates the number of words in the input, and returns a Result<>.
    pub fn validate_args(
        args: Vec<&str>,
        min_num_args_valid: i32,
        max_num_args_valid: i32,
    ) -> Result<bool, RunError> {
        let args_len = args.len() as i32;
        if args_len >= min_num_args_valid
            && (max_num_args_valid < 0 || args_len <= max_num_args_valid)
        {
            return Ok(true);
        }

        let msg_err = "Numero de argumentos inválido para el comando".to_string();
        Err(RunError {
            message: "ERR.".to_string(),
            cause: msg_err,
        })
    }
}

/// Tries to find the ending double quote of a given argument.
fn find_end_quote(
    pos: usize,
    request: Vec<&str>,
    mut string: String,
) -> Result<(usize, String), ParseError> {
    for (i, val) in request.iter().enumerate().skip(pos + 1) {
        string.push_str(request[i]);
        if val.to_string().contains('\"') {
            return Ok((i, string));
        }
    }

    Err(ParseError::quote_value(&"err"))
}

/// Validates that the request is correct in terms of arguments with double quotes and spaces.
fn validate_request(request: Vec<&str>) -> Result<Vec<String>, ParseError> {
    let mut validates_args = vec![];
    let mut pos = 0;

    while pos < request.len() {
        if !request[pos].is_empty() && request[pos] != " " {
            let arg = request[pos].split_ascii_whitespace().next().unwrap();
            if arg.to_string().contains('\"') {
                let vec = find_end_quote(pos, request.clone(), request[pos].to_string())?;
                pos = vec.0;
                validates_args.push(vec.1.trim_end().to_string());
            } else {
                validates_args.push(arg.to_string());
            }
        }

        pos += 1;
    }

    Ok(validates_args)
}

/// Validates the correctness of the first argument of the input, aka. the command.
pub fn obtain_str_command(msg: &str) -> Result<ParsedMessage, ParseError> {
    let msg_lower = String::from(msg).to_lowercase();
    if msg_lower.is_empty() {
        return Err(ParseError::empty_value(msg));
    }

    if msg_lower.trim().parse::<f64>().is_ok() {
        return Err(ParseError::numeric_value(msg_lower.as_str()));
    }

    let split_msg = msg_lower.split_inclusive(' ').collect();

    let mut _retrieved = vec![];
    if let Ok(value) = validate_request(split_msg) {
        _retrieved = value;
    } else {
        return Err(ParseError::quote_value(
            &"Text without final quote".to_string(),
        ));
    }

    let command = _retrieved[0].to_string();
    _retrieved.remove(0);
    let arguments = _retrieved;

    Ok(ParsedMessage { command, arguments })
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_and_one_argument() {
        let test_msg = "test_command test_single_argument";

        let result_msg = obtain_str_command(test_msg);
        assert_eq!(result_msg.is_ok(), true);

        let parsed_msg = result_msg.unwrap();
        assert_eq!(parsed_msg.command, "test_command");
        assert_eq!(parsed_msg.arguments, "test_single_argument");
    }

    #[test]
    fn test_parse_command_and_several_arguments() {
        let test_msg = "test_command test_argument_1 test_argument_2 test_argument_3";

        let result_msg = obtain_str_command(test_msg);
        assert_eq!(result_msg.is_ok(), true);

        let parsed_msg = result_msg.unwrap();
        assert_eq!(parsed_msg.command, "test_command");
        assert_eq!(
            parsed_msg.arguments,
            "test_argument_1 test_argument_2 test_argument_3"
        );
    }

    #[test]
    fn test_parse_command_and_no_arguments() {
        let test_msg = "test_command";

        let result_msg = obtain_str_command(test_msg);
        assert_eq!(result_msg.is_ok(), true);

        let parsed_msg = result_msg.unwrap();
        assert_eq!(parsed_msg.command, "test_command");
        assert_eq!(parsed_msg.arguments, "");
    }

    #[test]
    fn test_parse_empty_message() {
        let test_msg = "";

        let result_msg = obtain_str_command(test_msg);
        assert_eq!(result_msg.is_err(), true);
        assert_eq!(result_msg.err(), Some(ParseError::empty_value(test_msg)));
    }

    #[test]
    fn test_parse_numeric_command_message() {
        let test_msg = "9";

        let result_msg = obtain_str_command(test_msg);
        assert_eq!(result_msg.is_err(), true);
        assert_eq!(result_msg.err(), Some(ParseError::numeric_value(test_msg)));
    }

    #[test]
    fn test_parse_command_with_many_spaces() {
        let test_msg =
            "   test_command   test_argument_1    test_argument_2     test_argument_3     ";

        let result_msg = obtain_str_command(test_msg);
        assert_eq!(result_msg.is_ok(), true);

        let parsed_msg = result_msg.unwrap();
        assert_eq!(parsed_msg.command, "test_command");
        assert_eq!(
            parsed_msg.arguments,
            "test_argument_1 test_argument_2 test_argument_3"
        );
    }
}*/

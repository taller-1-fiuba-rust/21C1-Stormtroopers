use crate::errors::parse_error::ParseError;
use crate::errors::run_error::RunError;

pub struct ParsedMessage {
    pub command: String,
    pub arguments: String,
}

impl ParsedMessage {
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

pub fn obtain_str_command(msg: &str) -> Result<ParsedMessage, ParseError> {
    let msg_lower = String::from(msg).to_lowercase();
    if msg_lower.is_empty() {
        return Err(ParseError::empty_value(msg));
    }

    if msg_lower.trim().parse::<f64>().is_ok() {
        return Err(ParseError::numeric_value(msg_lower.as_str()));
    }

    let mut split_msg = msg_lower.split_whitespace();

    let command = String::from(split_msg.next().unwrap());
    let arguments = split_msg.fold(String::new(), |acc, x| {
        if acc.is_empty() {
            String::from(x)
        } else {
            format!("{}{}{}", acc, " ", x)
        }
    });

    Ok(ParsedMessage { command, arguments })
}

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
}

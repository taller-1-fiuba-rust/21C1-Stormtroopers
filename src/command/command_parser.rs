use crate::errors::parse_error::ParseError;

pub struct ParsedMessage {
    pub command: String,
    pub arguments: String,
}

pub fn obtain_str_command(msg: &str) -> Result<ParsedMessage, ParseError> {
    if msg.is_empty() {
        return Err(ParseError::empty_value(msg));
    }

    if msg.trim().parse::<f64>().is_ok() {
        return Err(ParseError::numeric_value(msg));
    }

    let mut split_msg = msg.split_whitespace();
    
    let command = String::from(split_msg.next().unwrap());
    let arguments = split_msg.fold(String::new(), |acc, x| {
        if acc.is_empty() {
            String::from(x)
        } else {
            format!("{}{}{}", acc, " ", x)
        }
    });

    return Ok(ParsedMessage {
        command,
        arguments,
    });
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


use std::error::Error;
use std::str::FromStr;
use std::io::{self, Read, Write};

use crate::{InputMode, ExecutionMode};
use crate::command::{Command, SubCommand};
use crate::csv::{DEFAULT_DELIMITER, DEFAULT_FILE_PATH};

pub fn get_command<R: Read>(_input_source: R, mode: ExecutionMode, args: &[String]) -> Result<Command, &'static str> {
    match mode {
        ExecutionMode::Interactive => {
            // Read the command from console in single-line mode:
            let command_input = match read_input(io::stdin(), InputMode::SingleLine, "a command".to_string()) {
                Ok(input) => input,
                Err(e) => format!("Error reading the command from the console: {}", e),
            };
            let mut command_args: Vec<String> = command_input.trim().split_whitespace().map(String::from).collect();

            command_args.insert(0, "".to_string());

            // Parse the command
            let command = parse_command(&command_args)?;
            Ok(command)
        },
        ExecutionMode::OneShot => {
            // Parse the command from the arguments:
            let command = match parse_command(&args) {
                Ok(command) => command,
                Err(e) => {
                    eprintln!("Error parsing command: {}", e);
                    println!("Although you haven't provided a valid command, you can still try to input something and see what happens.");
                    Command::NoCommand
                }
            };
            Ok(command)
        }
    }
}

pub fn parse_command(args: &[String]) -> Result<Command, &'static str> {
    match args.get(1) {
        Some(cmd) => Command::from_str(cmd.as_str()).map_err(|_| "Invalid command."),
        None => Err("No command provided."),
    }
}

pub fn get_csv_subcommand<R: Read>(_input_source: R, mode: ExecutionMode, args: &[String]) -> Result<SubCommand, &'static str> {
    match mode {
        ExecutionMode::Interactive => {
            // Read the settings from console in single-line mode:
            let settings_input = match read_input(io::stdin(), InputMode::SingleLine, "your CSV settings: p:<path> d:<delimiter> w:<max_column_width> \n(leave empty to enter in console)".to_string()) {
                Ok(input) => input,
                Err(_e) =>  return Ok(SubCommand::None), // no csv settings, so the user will input their csv into the console
            };
            let mut settings_args: Vec<String> = settings_input.trim().split_whitespace().map(String::from).collect();

            settings_args.insert(0, "".to_string());

            // Parse the csv settings
            let settings = parse_csv_settings(&settings_args)?;
            Ok(settings)
        },
        ExecutionMode::OneShot => {
            // Parse the csv settings from the arguments:
            let settings = match parse_csv_settings(&args) {
                Ok(settings) => settings,
                Err(_e) => return Ok(SubCommand::CsvSettings { path: Some(DEFAULT_FILE_PATH.to_string()), delimiter: Some(DEFAULT_DELIMITER), max_width: None })
            };
            Ok(settings)
        }
    }
}

pub fn read_input<R: Read>(_input_source: R, mode: InputMode, input_description: String) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();

    println!("Please enter {input_description}:");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    break;
                } else {
                    input.push_str(trimmed);
                    input.push('\n');
                }
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
        if mode == InputMode::SingleLine {
            break;
        }
    }

    if input.is_empty() {
        Err("No input provided.".into())
    } else {
        input.pop();
        Ok(input)
    }
}

pub fn parse_csv_settings(input: &[String]) -> Result<SubCommand, &'static str> {
    let mut path = None;
    let mut delimiter = None;
    let mut max_width = None;

    for part in input.iter().map(|s| s.trim()) {
        if part.starts_with("d:") {
            delimiter = part[2..].parse().ok();
        } else if part.starts_with("w:") {
            max_width = part[2..].parse().ok();
        } else if part.starts_with("p:") {
            path = part[2..].parse().ok();
        }
    }

    if path.is_none() && delimiter.is_none() && max_width.is_none() {
        Err("No valid settings provided.")
    } else {
        Ok(SubCommand::CsvSettings {
            path,
            delimiter,
            max_width,
        })
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::csv::Delimiter;

    #[test]
    fn wrong_command_test() {
        let args = vec!["prog_name".to_string(), "ultracase".to_string()];
        assert_eq!(parse_command(&args), Err("Invalid command."));
    }

    #[test]
    fn get_command_interactive_mode_test() {
        use std::io::Cursor;
    
        let input = Cursor::new("lowercase\n");
        let args = vec!["prog_name".to_string()];
    
        let command = get_command(input, ExecutionMode::Interactive, &args);
        assert_eq!(command, Ok(Command::Lowercase));
    }

    #[test]
    fn get_command_one_shot_mode_test() {
        use std::io::Cursor;
    
        let args = vec!["prog_name".to_string(), "lowercase".to_string()];
        let input = Cursor::new([]);
    
        let command = get_command(input, ExecutionMode::OneShot, &args);
        assert_eq!(command, Ok(Command::Lowercase));
    }

    #[test]
    fn get_csv_subcommand_interactive_mode_test() {
        use std::io::Cursor;
    
        let input = Cursor::new("p:example.csv d:; w:20\n");
        let args = vec!["prog_name".to_string()];
    
        let subcommand = get_csv_subcommand(input, ExecutionMode::Interactive, &args);
        assert_eq!(subcommand, Ok(SubCommand::CsvSettings { 
            path: Some("example.csv".to_string()), 
            delimiter: Some(Delimiter::Semicolon), 
            max_width: Some(20) 
        }));
    }

    #[test]
    fn get_csv_subcommand_one_shot_mode_test() {
        use std::io::Cursor;

        let args = vec!["prog_name".to_string(), "p:example.csv".to_string(), "d:;".to_string(), "w:20".to_string()];
        let input = Cursor::new([]);

        let subcommand = get_csv_subcommand(input, ExecutionMode::OneShot, &args);
        assert_eq!(subcommand, Ok(SubCommand::CsvSettings { 
            path: Some("example.csv".to_string()), 
            delimiter: Some(Delimiter::Semicolon), 
            max_width: Some(20) 
        }));
    }

    #[test]
    fn read_input_single_line_test() {
        use std::io::Cursor;
        let input = Cursor::new(b"Sample input\n");
    
        let result = read_input(input, InputMode::SingleLine, "Test input".to_string());
        if let Ok(value) = result {
            assert_eq!(value, "Sample input");
        } else {
            panic!("Expected Ok, got Err");
        }
    }

    #[test]
    fn parse_command_test() {
        let mut args = vec!["prog_name".to_string(), "lowercase".to_string()];
        assert_eq!(parse_command(&args), Ok(Command::Lowercase));
        args[1] = "no-spaces".to_string();
        assert_eq!(parse_command(&args), Ok(Command::NoSpaces));
        args[1] = "short-slugify".to_string();
        assert_eq!(parse_command(&args), Ok(Command::ShortSlugify));
        args[1] = "alternating".to_string();
        assert_eq!(parse_command(&args), Ok(Command::Alternating));
    }

}
//! Program that reads from standard input, transmutes text according to the provided specification, 
//! and prints the result back to the user. 
//! The behavior of the program is modified based on parsed CLI arguments.

// Rules:
// lowercase: convert the entire text to lowercase.
// uppercase: convert the entire text to uppercase.
// no-spaces: remove all spaces from the text.
// slugify: convert the text into a slug (a version of the text suitable for URLs) using the slug crate.
// short-slugify: convert the text into a short slug (similar to slugify but with a max length, cropped to the last dash before the length threshold).
// alternating: convert the text to an alternating between uppercase and lowercase pattern using the convert_case crate.
// leetify: Convert the text to leet speak using a .map() and a match block over specific letters.
// csv: parse the test as a CSV and print the data as a table. Usage: csv <delimiter> (defaults to semicolon). 
//      Put the delimiter in quotes to avoid shell expansion or other interpretation isues.

mod csv;
mod text_utils;

use std::str::FromStr;
use std::error::Error;
use std::io::{self, Write};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::fs::File;
use std::io::Read;

use csv::{Delimiter, DEFAULT_DELIMITER, DEFAULT_COL_WIDTH};
use text_utils::{lowercase, uppercase, no_spaces, slugify, short_slugify, alternating, leetify};

const DEFAULT_FILE_PATH: &str = "test/username.csv"; 

#[derive(Debug, PartialEq)]
enum Command {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    ShortSlugify,
    Alternating,
    Leetify,
    Csv,
    NoCommand,
}

#[derive(PartialEq)]
enum SubCommand {
    CsvSettings {
        path: Option<String>,
        delimiter: Option<Delimiter>,
        max_width: Option<i32>,
    },
    None
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "lowercase" => Ok(Command::Lowercase),
            "uppercase" => Ok(Command::Uppercase),
            "no-spaces" => Ok(Command::NoSpaces),
            "slugify" => Ok(Command::Slugify),
            "short-slugify" => Ok(Command::ShortSlugify),
            "alternating" => Ok(Command::Alternating),
            "leetify" => Ok(Command::Leetify),
            "csv" => Ok(Command::Csv),
            _ => Err(()),
        }
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Lowercase => "lowercase".to_string(),
            Command::Uppercase => "uppercase".to_string(),
            Command::NoSpaces => "no-spaces".to_string(),
            Command::Slugify => "slugify".to_string(),
            Command::ShortSlugify => "short-slugify".to_string(),
            Command::Alternating => "alternating".to_string(),
            Command::Leetify => "leetify".to_string(),
            Command::Csv => "csv".to_string(),
            Command::NoCommand => "no command".to_string(),
        }
    }
}

#[derive(PartialEq)]
enum InputMode {
    SingleLine,
    MultiLine,
}

#[derive(PartialEq, Clone, Copy)]
enum ExecutionMode {
    OneShot,
    Interactive,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let execution_mode = match args.len() {
        1 => ExecutionMode::Interactive,
        _ => ExecutionMode::OneShot,
    };

    let (tx, rx) = channel(); // data channel
    let (done_tx, done_rx) = channel(); // sync channel

    let producer = thread::spawn( {
        let args = args.clone();
        move || run_producer(&tx, &done_rx, execution_mode, &args)
    });

    let consumer = thread::spawn(move || {
        while let Ok((command, subcommand, input)) = rx.recv() {
            execute_command(command, subcommand, input);
            done_tx.send(()).unwrap();
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

fn run_producer(tx: &Sender<(Command, SubCommand, String)>, done_rx: &Receiver<()>, mode: ExecutionMode, args: &[String]) {
    let mut ran_once = false;

    while mode == ExecutionMode::Interactive || !ran_once {
        if let Ok(command) = get_command(mode, args) {
            let mut subcommand: SubCommand = SubCommand::None;
            if command == Command::Csv {
                if let Ok(attempted_subcommand) = get_csv_subcommand(mode, args) {
                    subcommand = attempted_subcommand;
                }
            }
            if subcommand == SubCommand::None {
                let input_description = format!("your input to {}", command.to_string());
                if let Ok(input) = read_input(InputMode::MultiLine, input_description) {
                    if tx.send((command, subcommand, input)).is_err() {
                        eprintln!("Error sending data to consumer.");
                        break;
                    }
                } else {
                    eprintln!("Error reading input.");
                    break;
                }
            } else {
                if tx.send((command, subcommand, "".to_string())).is_err() {
                    eprintln!("Error sending data to consumer.");
                    break;
                }
            }

            if done_rx.recv().is_err() {
                eprintln!("Consumer shut down with an error.");
                break;
            }
        } else {
            eprintln!("Error getting a command.");
            break;
        }
        ran_once = true; 
    }
}

fn execute_command(command: Command, subcommand: SubCommand, input: String) {
    match transmute(input, command, subcommand) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error executing command: {}", e),
    }
}

fn get_command(mode: ExecutionMode, args: &[String]) -> Result<Command, &'static str> {
    match mode {
        ExecutionMode::Interactive => {
            // Read the command from console in single-line mode:
            let command_input = match read_input(InputMode::SingleLine, "a command".to_string()) {
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

fn get_csv_subcommand(mode: ExecutionMode, args: &[String]) -> Result<SubCommand, &'static str> {
    match mode {
        ExecutionMode::Interactive => {
            // Read the settings from console in single-line mode:
            let settings_input = match read_input(InputMode::SingleLine, "your CSV settings: p:<path> d:<delimiter> w:<max_column_width> (if no path is provided - enter csv in the console)".to_string()) {
                Ok(input) => input,
                Err(e) => return Ok(SubCommand::CsvSettings { path: None, delimiter: Some(DEFAULT_DELIMITER), max_width: None }), // empty input -> default
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
                Err(e) => return Ok(SubCommand::CsvSettings { path: None, delimiter: Some(DEFAULT_DELIMITER), max_width: None })
            };
            Ok(settings)
        }
    }
}

fn parse_csv_settings(input: &[String]) -> Result<SubCommand, &'static str> {
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

fn parse_command(args: &[String]) -> Result<Command, &'static str> {
    match args.get(1) {
        Some(cmd) => Command::from_str(cmd.as_str()).map_err(|_| "Invalid command."),
        None => Err("No command provided."),
    }
}

fn read_input(mode: InputMode, input_description: String) -> Result<String, Box<dyn Error>> {
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

fn transmute(string: String, command: Command, subcommand: SubCommand) -> Result<String, Box<dyn Error>> {
    match command {
        Command::Lowercase => lowercase(string),
        Command::Uppercase => uppercase(string),
        Command::NoSpaces => no_spaces(string),
        Command::Slugify => slugify(&string),
        Command::ShortSlugify => short_slugify(string),
        Command::Alternating => alternating(string),
        Command::Leetify => leetify(string),
        Command::Csv => process_csv(string, subcommand),
        // Command::NoCommand => Err("No valid command provided".into()), // An alternative way of handling the case where no command is provided.
        Command::NoCommand => no_command(string),
    }
} 

fn process_csv(s: String, subcommand: SubCommand) -> Result<String, Box<dyn Error>> {
    let mut csv = csv::Csv::new();

    if let SubCommand::CsvSettings { path, delimiter, max_width } = subcommand {
        let mut csv_data = s;
        if let Some(file_path) = path {
            match csv_string_from_file(file_path) {
                Ok(file_content) => csv_data = file_content,
                Err(e) => return Err(e),
            }
        }
        
        csv.parse_csv_data(&csv_data, delimiter.unwrap_or(DEFAULT_DELIMITER))?;
        Ok(csv.to_string())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid csv settings.")))
    }
}

fn csv_string_from_file(path: String) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read the file");
    Ok(content)
}

fn identify_delimiter(s: Option<&str>) -> Result<Delimiter, Box<dyn Error>> {
    match s {
        Some(",") => Ok(Delimiter::Comma),
        Some(";") => Ok(Delimiter::Semicolon),
        _ => Err("Invalid delimiter.".into()),
    }
}

fn no_command(mut s: String) -> Result<String, Box<dyn Error>> {
        #[cfg(not(test))]
        {
            println!("You aren't using this program properly, but here's an output anyway. I put a little something there so you don't feel bad.");
        }
        s.push('🧁');
        Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // fn transmute_test() {
    //     let args: Vec<String> = vec![];
    //     assert_eq!(transmute(
    //         "Crabs can be found in all oceans and in fresh water.".to_string(), 
    //         Command::Lowercase, &args).unwrap(),
    //         "crabs can be found in all oceans and in fresh water.");
    //     assert_eq!(transmute(
    //         "There are at least 7,000 species, or kinds, of crab!".to_string(), 
    //         Command::Uppercase, &args).unwrap(),
    //         "THERE ARE AT LEAST 7,000 SPECIES, OR KINDS, OF CRAB!");
    //     assert_eq!(transmute(
    //         "Crabs and other crustaceans have a hard covering known as the exoskeleton.".to_string(), 
    //         Command::NoSpaces, &args).unwrap(),
    //         "Crabsandothercrustaceanshaveahardcoveringknownastheexoskeleton.");
    //     assert_eq!(transmute(
    //         "Crabs breathe by using gills, but the gills of land crabs have developed in such a way that they act like lungs.".to_string(), 
    //         Command::Slugify, &args).unwrap(),
    //         "crabs-breathe-by-using-gills-but-the-gills-of-land-crabs-have-developed-in-such-a-way-that-they-act-like-lungs");
    //     assert_eq!(transmute(
    //         "Tiny pea crabs may measure less than an inch (2.5 centimeters) across.".to_string(), 
    //         Command::ShortSlugify, &args).unwrap(),
    //         "tiny-pea-crabs");
    //     assert_eq!(transmute(
    //         "Some types, including the blue crab, the Dungeness crab, and the king crab, are often eaten by humans. Crabs may be sold fresh to restaurants or their meat may be canned.".to_string(), 
    //         Command::Alternating, &args).unwrap(),
    //         "sOmE tYpEs, InClUdInG tHe BlUe CrAb, ThE dUnGeNeSs CrAb, AnD tHe KiNg CrAb, ArE oFtEn EaTeN bY hUmAnS. cRaBs MaY bE sOlD fReSh To ReStAuRaNtS oR tHeIr MeAt MaY bE cAnNeD.");
    //     assert_eq!(transmute(
    //         "As the crab grows larger, it seeks a larger shell.".to_string(), 
    //         Command::Leetify, &args).unwrap(),
    //         "45 7H3 CR48 9R0W5 L4R93R, 17 533K5 4 L4R93R 5H3LL.");
    //     assert_eq!(transmute(
    //          "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.".to_string(), Command::NoCommand, &args).unwrap(),
    //          "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.🧁");
    // }

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

    // #[test]
    // fn wrong_command_test() {
    //     let args = vec!["prog_name".to_string(), "ultracase".to_string()];
    //     assert_eq!(parse_command(&args), Err("Invalid command."));
    //     assert_eq!(transmute("a crab has five pairs of legs".to_string(), Command::NoCommand, &args).unwrap(), "a crab has five pairs of legs🧁");
    // }

    #[test]
    fn no_command_test() {
        assert_eq!(no_command("One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.".to_string()).unwrap(),
                   "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.🧁");
    }
}
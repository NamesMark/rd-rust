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
// csv: parse the test as a CSV and print the data as a table. Usage: p:<path> d:<delimiter> w:<max_column_width>. 

mod command;
mod csv;
mod text_utils;
mod input;
mod csv_process;

use std::error::Error;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::io::{self};

use command::{Command, SubCommand};
use text_utils::{lowercase, uppercase, no_spaces, slugify, short_slugify, alternating, leetify};
use input::{get_command, get_csv_subcommand, read_input};
use csv_process::process_csv;


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
        if let Ok(command) = get_command(io::stdin(), mode, args) {
            let mut subcommand: SubCommand = SubCommand::None;
            if command == Command::Csv {
                if let Ok(attempted_subcommand) = get_csv_subcommand(io::stdin(), mode, args) {
                    subcommand = attempted_subcommand;
                }
            }
            if subcommand == SubCommand::None {
                let input_description = format!("your input to {}", command.to_string());
                if let Ok(input) = read_input(io::stdin(), InputMode::MultiLine, input_description) {
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
        Command::NoCommand => no_command(string),
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
    fn transmute_test() {
        assert_eq!(transmute(
            "Crabs can be found in all oceans and in fresh water.".to_string(), 
            Command::Lowercase, SubCommand::None).unwrap(),
            "crabs can be found in all oceans and in fresh water.");
        assert_eq!(transmute(
            "There are at least 7,000 species, or kinds, of crab!".to_string(), 
            Command::Uppercase, SubCommand::None).unwrap(),
            "THERE ARE AT LEAST 7,000 SPECIES, OR KINDS, OF CRAB!");
        assert_eq!(transmute(
            "Crabs and other crustaceans have a hard covering known as the exoskeleton.".to_string(), 
            Command::NoSpaces, SubCommand::None).unwrap(),
            "Crabsandothercrustaceanshaveahardcoveringknownastheexoskeleton.");
        assert_eq!(transmute(
            "Crabs breathe by using gills, but the gills of land crabs have developed in such a way that they act like lungs.".to_string(), 
            Command::Slugify, SubCommand::None).unwrap(),
            "crabs-breathe-by-using-gills-but-the-gills-of-land-crabs-have-developed-in-such-a-way-that-they-act-like-lungs");
        assert_eq!(transmute(
            "Tiny pea crabs may measure less than an inch (2.5 centimeters) across.".to_string(), 
            Command::ShortSlugify, SubCommand::None).unwrap(),
            "tiny-pea-crabs");
        assert_eq!(transmute(
            "Some types, including the blue crab, the Dungeness crab, and the king crab, are often eaten by humans. Crabs may be sold fresh to restaurants or their meat may be canned.".to_string(), 
            Command::Alternating, SubCommand::None).unwrap(),
            "sOmE tYpEs, InClUdInG tHe BlUe CrAb, ThE dUnGeNeSs CrAb, AnD tHe KiNg CrAb, ArE oFtEn EaTeN bY hUmAnS. cRaBs MaY bE sOlD fReSh To ReStAuRaNtS oR tHeIr MeAt MaY bE cAnNeD.");
        assert_eq!(transmute(
            "As the crab grows larger, it seeks a larger shell.".to_string(), 
            Command::Leetify, SubCommand::None).unwrap(),
            "45 7H3 CR48 9R0W5 L4R93R, 17 533K5 4 L4R93R 5H3LL.");
        assert_eq!(transmute(
            "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.".to_string(), 
            Command::NoCommand, SubCommand::None).unwrap(),
            "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.🧁");
    }

    #[test]
    fn no_command_test() {
        assert_eq!(no_command("One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.".to_string()).unwrap(),
                   "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.🧁");
    }
}
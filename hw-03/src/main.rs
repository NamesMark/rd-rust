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

extern crate slug;
use slug::slugify;

use std::str::FromStr;
use std::error::Error;

use convert_case::{Case, Casing};

const MAX_SHORT_SLUG: usize = 16;
const MIN_SHORT_SLUG: usize = 5;

#[derive(Debug, PartialEq)]
enum Command {
    Lowercase,
    Uppercase,
    NoSpaces,
    Slugify,
    ShortSlugify,
    Alternating,
    Leetify,
    NoCommand,
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
            _ => Err(()),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let command = match parse_command(&args) {
        Ok(command) => command,
        Err(e) => {
            eprintln!("Error parsing command: {}", e);
            Command::NoCommand
        }
    };

    let input = match read_input() {
        Ok(input) => input,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return;
        }
    };

    match transmute(input, command) {
        Ok(result) => println!("{}", result),
        Err(e) => eprintln!("Error executing command: {}", e),
    };
}

fn parse_command(args: &[String]) -> Result<Command, &'static str> {
    match args.get(1) {
        Some(cmd) => Command::from_str(cmd.as_str()).map_err(|_| "Invalid command"),
        None => Ok(Command::NoCommand),
    }
}

fn read_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    let mut counter: usize = 0;
    while counter < 5 {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim().to_string();
                if !trimmed.is_empty() {
                    return Ok(trimmed);
                }
                println!("No input provided. Please try again.");
                eprintln!("Error reading input during attempt {}: {}", counter, "No input provided.".to_string());
            }
            Err(e) => eprintln!("Error reading input: {}", e),
        }
        counter += 1;
    }
    return Err("Too many failed attempts to read input.".into());
    //panic!("Too many failed attempts to read input.");
}

fn transmute(string: String, command: Command) -> Result<String, Box<dyn Error>> {
    match command {
        Command::Lowercase => lowercase(string),
        Command::Uppercase => uppercase(string),
        Command::NoSpaces => no_spaces(string),
        Command::Slugify => slug(&string),
        Command::ShortSlugify => short_slugify(string),
        Command::Alternating => alternating(string),
        Command::Leetify => leetify(string),
        Command::NoCommand => Err("No valid command provided".into()),
    }
} 

fn lowercase(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_lowercase())
}

fn uppercase(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_uppercase())
}

fn no_spaces(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.replace(" ", ""))
}

fn slug(s: &str) -> Result<String, Box<dyn Error>> {
    Ok(slug::slugify(s))
}

fn short_slugify(string: String) -> Result<String, Box<dyn Error>> {
    let short_slug = slugify(&string).chars().take(MAX_SHORT_SLUG).collect::<String>();
    let mut trimmed_short_slug = short_slug.clone();
    while !trimmed_short_slug.ends_with('-') && !trimmed_short_slug.is_empty() {
        trimmed_short_slug.pop();
    }
    let trimmed_short_slug = trimmed_short_slug.trim_end_matches('-').to_string();
    if trimmed_short_slug.len() < MIN_SHORT_SLUG {
        return Ok(short_slug);
    }
    
    Ok(trimmed_short_slug)
}

fn alternating(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_case(Case::Alternating))
}

fn leetify(s: String) -> Result<String, Box<dyn Error>> {
    Ok(s.to_uppercase().chars().map(|c| match c {
        'A' => '4',
        'B' => '8',
        'E' => '3',
        'G' => '9',
        'I' => '1',
        'O' => '0',
        'S' => '5',
        'T' => '7',
        _ => c,
    }).collect())
}

fn no_command(mut s: String) -> String {
        #[cfg(not(test))]
        {
            println!("You aren't using this program properly, but here's an output anyway. I put a little something there so you don't feel bad.");
        }
        s.push('🧁');
        s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmute_test() {
        assert_eq!(transmute(
            "Crabs can be found in all oceans and in fresh water.".to_string(), Command::Lowercase).unwrap(),
            "crabs can be found in all oceans and in fresh water.");
        assert_eq!(transmute(
            "There are at least 7,000 species, or kinds, of crab!".to_string(), Command::Uppercase).unwrap(),
            "THERE ARE AT LEAST 7,000 SPECIES, OR KINDS, OF CRAB!");
        assert_eq!(transmute(
            "Crabs and other crustaceans have a hard covering known as the exoskeleton.".to_string(), Command::NoSpaces).unwrap(),
            "Crabsandothercrustaceanshaveahardcoveringknownastheexoskeleton.");
        assert_eq!(transmute(
            "Crabs breathe by using gills, but the gills of land crabs have developed in such a way that they act like lungs.".to_string(), Command::Slugify).unwrap(),
            "crabs-breathe-by-using-gills-but-the-gills-of-land-crabs-have-developed-in-such-a-way-that-they-act-like-lungs");
        assert_eq!(transmute(
            "Tiny pea crabs may measure less than an inch (2.5 centimeters) across.".to_string(), Command::ShortSlugify).unwrap(),
            "tiny-pea-crabs");
        assert_eq!(transmute(
            "Some types, including the blue crab, the Dungeness crab, and the king crab, are often eaten by humans. Crabs may be sold fresh to restaurants or their meat may be canned.".to_string(), 
            Command::Alternating).unwrap(),
            "sOmE tYpEs, InClUdInG tHe BlUe CrAb, ThE dUnGeNeSs CrAb, AnD tHe KiNg CrAb, ArE oFtEn EaTeN bY hUmAnS. cRaBs MaY bE sOlD fReSh To ReStAuRaNtS oR tHeIr MeAt MaY bE cAnNeD.");
        assert_eq!(transmute(
            "As the crab grows larger, it seeks a larger shell.".to_string(), 
            Command::Leetify).unwrap(),
            "45 7H3 CR48 9R0W5 L4R93R, 17 533K5 4 L4R93R 5H3LL.");
        // assert_eq!(transmute(
        //     "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.".to_string(), Command::NoCommand).unwrap(),
        //     "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.🧁");
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

    #[test]
    fn short_slugify_test() {
        assert_eq!(short_slugify("Crabs can be found in all oceans and in fresh water.".to_string()).unwrap(), "crabs-can-be");
        assert_eq!(short_slugify("Crabsandothercrustaceanshaveahard covering known as the exoskeleton.".to_string()).unwrap(), "crabsandothercru");
        assert_eq!(short_slugify("Although a few baby crabs leave the egg looking like small adults, most do not.".to_string()).unwrap(), "although-a-few");
    }

    #[test]
    fn wrong_command_test() {
        let args = vec!["prog_name".to_string(), "ultracase".to_string()];
        assert_eq!(parse_command(&args), Err("Invalid command"));
        // assert_eq!(transmute("a crab has five pairs of legs".to_string(), Command::NoCommand).unwrap(), "a crab has five pairs of legs🧁");
    }
}
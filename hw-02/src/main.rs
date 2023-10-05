//! Program that reads from standard input, transmutes text according to the provided specification, 
//! and prints the result back to the user. 
//! The behavior of the program is modified based on parsed CLI arguments.

// Rules:
// lowercase: convert the entire text to lowercase.
// uppercase: convert the entire text to uppercase.
// no-spaces: remove all spaces from the text.
// slugify: convert the text into a slug (a version of the text suitable for URLs) using the slug crate.
// short-slugify: convert the text into a short slug (similar to slugify but with a max length, cropped to the last dash before the length threshold).
// alternating: Convert the text to an alternating between uppercase and lowercase pattern using the convert_case crate.

extern crate slug;
use slug::slugify;
use std::str::FromStr;
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
            _ => Err(()),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = read_input();
    let command = parse_command(&args);
    let result = transmute(input, command);
    println!("{}", result);
}

fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_command(args: &[String]) -> Command {
    match args.get(1) {
        Some(cmd) => Command::from_str(cmd.as_str()).unwrap_or(Command::NoCommand),
        None => Command::NoCommand
    }
}

fn transmute(string: String, command: Command) -> String {
    match command {
        Command::Lowercase => string.to_lowercase(),
        Command::Uppercase => string.to_uppercase(),
        Command::NoSpaces => string.replace(" ", ""),
        Command::Slugify => slugify(&string),
        Command::ShortSlugify => short_slugify(string),
        Command::Alternating => string.to_case(Case::Alternating),
        Command::NoCommand => {
            println!("You aren't using this program properly, but here's an output anyway. I put a little something there so you don't feel bad.");
            let mut string = string;
            string.push('🧁');
            string
        }
    }
} 

fn short_slugify(string: String) -> String {
    let short_slug = slugify(&string).chars().take(MAX_SHORT_SLUG).collect::<String>();
    let mut trimmed_short_slug = short_slug.clone();
    while !trimmed_short_slug.ends_with('-') && !trimmed_short_slug.is_empty() {
        trimmed_short_slug.pop();
    }
    let trimmed_short_slug = trimmed_short_slug.trim_end_matches('-').to_string();
    if trimmed_short_slug.len() < MIN_SHORT_SLUG {
        return short_slug;
    }
    
    trimmed_short_slug
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transmute_test() {
        assert_eq!(transmute(
            "Crabs can be found in all oceans and in fresh water.".to_string(), Command::Lowercase),
            "crabs can be found in all oceans and in fresh water.");
        assert_eq!(transmute(
            "There are at least 7,000 species, or kinds, of crab!".to_string(), Command::Uppercase),
            "THERE ARE AT LEAST 7,000 SPECIES, OR KINDS, OF CRAB!");
        assert_eq!(transmute(
            "Crabs and other crustaceans have a hard covering known as the exoskeleton.".to_string(), Command::NoSpaces),
            "Crabsandothercrustaceanshaveahardcoveringknownastheexoskeleton.");
        assert_eq!(transmute(
            "Crabs breathe by using gills, but the gills of land crabs have developed in such a way that they act like lungs.".to_string(), Command::Slugify),
            "crabs-breathe-by-using-gills-but-the-gills-of-land-crabs-have-developed-in-such-a-way-that-they-act-like-lungs");
        assert_eq!(transmute(
            "Tiny pea crabs may measure less than an inch (2.5 centimeters) across.".to_string(), Command::ShortSlugify),
            "tiny-pea-crabs");
        assert_eq!(transmute(
            "Some types, including the blue crab, the Dungeness crab, and the king crab, are often eaten by humans. Crabs may be sold fresh to restaurants or their meat may be canned.".to_string(), 
            Command::Alternating),
            "sOmE tYpEs, InClUdInG tHe BlUe CrAb, ThE dUnGeNeSs CrAb, AnD tHe KiNg CrAb, ArE oFtEn EaTeN bY hUmAnS. cRaBs MaY bE sOlD fReSh To ReStAuRaNtS oR tHeIr MeAt MaY bE cAnNeD.");
        assert_eq!(transmute(
            "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.".to_string(), Command::NoCommand),
            "One group of crabs, the hermits, are known for their habit of taking over empty snail shells for shelter.🧁");
    }

    #[test]
    fn parse_command_test() {
        let mut args = vec!["prog_name".to_string(), "lowercase".to_string()];
        assert_eq!(parse_command(&args), Command::Lowercase);
        args[1] = "no-spaces".to_string();
        assert_eq!(parse_command(&args), Command::NoSpaces);
        args[1] = "short-slugify".to_string();
        assert_eq!(parse_command(&args), Command::ShortSlugify);
        args[1] = "alternating".to_string();
        assert_eq!(parse_command(&args), Command::Alternating);
    }

    #[test]
    fn short_slugify_test() {
        assert_eq!(short_slugify("Crabs can be found in all oceans and in fresh water.".to_string()), "crabs-can-be");
        assert_eq!(short_slugify("Crabsandothercrustaceanshaveahard covering known as the exoskeleton.".to_string()), "crabsandothercru");
        assert_eq!(short_slugify("Although a few baby crabs leave the egg looking like small adults, most do not.".to_string()), "although-a-few");
    }

    #[test]
    fn wrong_command_test() {
        let args = vec!["prog_name".to_string(), "ultracase".to_string()];
        assert_eq!(parse_command(&args), Command::NoCommand);
        assert_eq!(transmute("a crab has five pairs of legs".to_string(), Command::NoCommand), "a crab has five pairs of legs🧁");
    }
}
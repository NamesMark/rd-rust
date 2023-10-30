
use std::str::FromStr;

use crate::csv::Delimiter;

#[derive(Debug, PartialEq)]
pub enum Command {
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

#[derive(Debug, PartialEq)]
pub enum SubCommand {
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
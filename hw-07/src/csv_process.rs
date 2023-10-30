
use std::error::Error;
use std::fs::File;
use std::io::Read;

use crate::csv::{self, DEFAULT_COL_WIDTH};

use crate::command::SubCommand;
use csv::DEFAULT_DELIMITER;

pub fn process_csv(s: String, subcommand: SubCommand) -> Result<String, Box<dyn Error>> {
    let mut csv = csv::Csv::new();

    // csv string is provided through the console interactively:
    if !s.is_empty() {
        csv.parse_csv_data(&s, DEFAULT_DELIMITER, DEFAULT_COL_WIDTH)?;
        return Ok(csv.to_string());
    }

    // csv is provided through the file:
    if let SubCommand::CsvSettings { path, delimiter, max_width } = subcommand {
        let mut csv_data = s;
        if let Some(file_path) = path {
            match csv_string_from_file(file_path) {
                Ok(file_content) => csv_data = file_content,
                Err(e) => return Err(e),
            }
        }
        let width: usize;
        if let Some(w) = max_width {
            width = w as usize;
        } else {
            width = DEFAULT_COL_WIDTH;
        }
        
        csv.parse_csv_data(&csv_data, delimiter.unwrap_or(DEFAULT_DELIMITER), width)?;
        Ok(csv.to_string())
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid csv settings.")))
    }
}

pub fn csv_string_from_file(path: String) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Unable to read the file");
    Ok(content)
}
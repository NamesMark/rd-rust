use std::{str::FromStr, fmt::Display};

use pad::{PadStr, Alignment};

const MAX_COLUMN_CAPACITY: usize = 16;

pub enum Delimiter {
    Comma,
    Semicolon
}

impl FromStr for Delimiter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "," => Ok(Delimiter::Comma),
            ";" => Ok(Delimiter::Semicolon),
            _ => Err(()),
        }
    }
}

impl Display for Delimiter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Delimiter::Comma => write!(f, ","),
            Delimiter::Semicolon => write!(f, ";"),
        }
    }
}

pub struct Csv {
    columns: Vec<String>,
    data: Vec<Vec<String>>,
}

impl Csv {
    pub fn new() -> Self {
        Self {
            columns: vec!(String::new()),
            data: vec!(vec!(String::new())),
        }
    }
    
    pub fn parse_csv_data(&mut self, data: &str, delimiter: Delimiter) -> Result<(), String> {
        let lines:Vec<&str> = data.split('\n').collect();
        if lines.is_empty() {
            return Err("No csv data found.".to_string());
        }

        let headers: Vec<String> = lines[0]
            .split(&delimiter.to_string())
            .map(|header| { 
                let mut header = header.to_string();
                header.truncate(MAX_COLUMN_CAPACITY);
                header
            })
            .collect();
        if headers.is_empty() {
            return Err("No header data found.".to_string()); 
        }
        self.columns = headers;
            
        let data: Vec<Vec<String>> = lines[1..]
            .iter()
            .map(|line| {
                line.split(&delimiter.to_string())
                    .map(|value| { 
                        let mut value = value.to_string();
                        value.truncate(MAX_COLUMN_CAPACITY);
                        value
                    })
                    .collect()
            })
            .collect();
        if data.is_empty() {
            return Err("No data found".to_string());
        }
        self.data = data;
        
        Ok(())
    }

    pub fn display_csv_data(&self) {
        let column_num = (self.columns).len();

        let mut base_line = format!("{}┬", "─".repeat(MAX_COLUMN_CAPACITY))
        .repeat(column_num);
        base_line.pop();

        let top_line =    format!("╭{}╮", base_line);
        let header_line = format!("╞{}╡", base_line.replace("─", "═").replace("┬", "╪"));
        let middle_line = format!("│{}│", base_line.replace("┬", "┼"));
        let bottom_line = format!("╰{}╯", base_line.replace("┬", "┴"));
        println!("{}", top_line);

        for column_name in self.columns.iter() {
            print!("{}", "│");
            // print!("{:MAX_COLUMN_CAPACITY$}", column_name); // one way to pad
            print!("{}", column_name.pad_to_width_with_alignment(MAX_COLUMN_CAPACITY, Alignment::Middle));
        }
        println!("{}", "│");
        println!("{}", header_line);

        for (index, row) in self.data.iter().enumerate() {
            print!("{}", "│");
            for (index, value) in row.iter().enumerate() {
                print!("{}", value.pad_to_width_with_alignment(MAX_COLUMN_CAPACITY, Alignment::Middle));
                if (index + 1) < row.len() {
                    print!("{}", "┆");
                }
            }
            println!("{}", "│");
            if (index + 1) < self.data.len() {
                println!("{}", middle_line);
            }
        }
        println!("{}", bottom_line);
    }
}
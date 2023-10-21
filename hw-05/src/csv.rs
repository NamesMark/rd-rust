use std::{str::FromStr, fmt::Display};

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
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}

impl Csv {
    pub fn new() -> Self {
        Self {
            headers: vec!(String::new()),
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
                header.truncate(16);
                header
            })
            .collect();
        if headers.is_empty() {
            return Err("No header data found.".to_string()); 
        }
        self.headers = headers;
            
        let data: Vec<Vec<String>> = lines[1..]
            .iter()
            .map(|line| {
                line.split(&delimiter.to_string())
                    .map(|value| { 
                        let mut value = value.to_string();
                        value.truncate(16);
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

    // pub fn parse_csv_data_default(&mut self, data: &str) -> Result<(), String> {
    //     let default_delimiter = Delimiter::Semicolon;
    //     self.parse_csv_data(data, default_delimiter)
    // }

    pub fn display_csv_data(&self) {
        for (index, header) in self.headers.iter().enumerate() {
            println!("{}: {}", index, header);
        }
        for row in self.data.iter() {
            for (index, value) in row.iter().enumerate() {
                println!("{}: {}", index, value);
            }
        }
    }
}
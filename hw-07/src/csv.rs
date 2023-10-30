use std::str::FromStr;
use std::fmt;

pub const DEFAULT_DELIMITER: Delimiter = Delimiter::Semicolon;
pub const DEFAULT_COL_WIDTH: usize = 16;
pub const DEFAULT_FILE_PATH: &str = "test/username.csv"; 
pub const MAX_COLUMN_CAPACITY: usize = DEFAULT_COL_WIDTH;

#[derive(Debug, PartialEq)]
pub enum Delimiter {
    Comma,
    Semicolon
}

impl FromStr for Delimiter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "," | "comma" => Ok(Delimiter::Comma),
            ";" | "semicolon" => Ok(Delimiter::Semicolon),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Delimiter {
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
    
    pub fn parse_csv_data(&mut self, data: &str, delimiter: Delimiter, max_width: usize) -> Result<(), String> {
        let lines:Vec<&str> = data.split('\n').collect();
        if lines.is_empty() {
            return Err("No csv data found.".to_string());
        }

        let headers: Vec<String> = lines[0]
            .split(&delimiter.to_string())
            .map(|header| { 
                let mut header = header.to_string();
                header.truncate(max_width);
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
                        value.truncate(max_width);
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

    pub fn format_as_table(&self) -> String {
        let mut string_table = String::new();
        let column_num = (self.columns).len();

        let mut base_line = format!("{}┬", "─".repeat(MAX_COLUMN_CAPACITY))
        .repeat(column_num);
        base_line.pop();

        let top_line =    format!("╭{}╮\n", base_line);
        let header_line = format!("╞{}╡\n", base_line.replace("─", "═").replace("┬", "╪"));
        let middle_line = format!("│{}│\n", base_line.replace("┬", "┼"));
        let bottom_line = format!("╰{}╯\n", base_line.replace("┬", "┴"));

        string_table.push_str(&top_line);

        for column_name in self.columns.iter() {
            string_table.push_str("│");
            string_table.push_str(&format!("{:^width$}", column_name, width = MAX_COLUMN_CAPACITY));
        }
        string_table.push_str("│\n");
        string_table.push_str(&header_line);

        for (index, row) in self.data.iter().enumerate() {
            string_table.push_str("│");
            for (index, value) in row.iter().enumerate() {
                string_table.push_str(&format!("{:^width$}", value, width = MAX_COLUMN_CAPACITY));
                if (index + 1) < row.len() {
                    string_table.push_str("┆");
                }
            }
            string_table.push_str("│\n");
            if (index + 1) < self.data.len() {
                string_table.push_str(&middle_line);
            }
        }
        
        string_table.push_str(&bottom_line);

        string_table
    }

}

impl fmt::Display for Csv{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_as_table())
    }
}
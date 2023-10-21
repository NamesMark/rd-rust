use std::str::FromStr;
use std::fmt;

//use pad::{PadStr, Alignment};

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

    fn get_max_column_widths(&self) -> Vec<usize> {
        let mut max_widths = vec![0; self.columns.len()];
        
        for (i, column) in self.columns.iter().enumerate() {
            max_widths[i] = column.chars().count();
        }

        for row in &self.data {
            for (i, cell) in row.iter().enumerate() {
                max_widths[i] = max_widths[i].max(cell.chars().count());
            }
        }

        max_widths
    }

    fn wrap_text(&self, text: &str, max_width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut line = String::new();

        for word in text.split_whitespace() {
            if line.len() + word.len() + 1 > max_width {
                lines.push(line);
                line = String::new();
            }
            if !line.is_empty() {
                line.push(' ');
            }
            line.push_str(word);
        }
        if !line.is_empty() {
            lines.push(line);
        }

        lines
    }

    pub fn format_as_table(&self) -> String {
        let max_widths = self.get_max_column_widths();
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
            let mut wrapped_lines: Vec<Vec<String>> = Vec::new();

            for (i, cell) in row.iter().enumerate() {
                let lines = self.wrap_text(cell, max_widths[i]);
                while wrapped_lines.len() < lines.len() {
                    wrapped_lines.push(vec!["".to_string(); row.len()]);
                }
                for (j, line) in lines.iter().enumerate() {
                    wrapped_lines[j][i] = line.clone();
                }
            }

            for lines in &wrapped_lines {
                for (i, line) in lines.iter().enumerate() {
                    string_table.push_str("│");
                    string_table.push_str(&format!("{:^width$}", line, width = max_widths[i]));
                }
                string_table.push_str("│\n");
            }
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
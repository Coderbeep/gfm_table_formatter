// # of cells in separator row must be equal to # of cells in header row
// no content (only headers) is accepted => min. # of rows is 2 - header and separator
// escape delimiter
#![allow(unused_variables)]

use std::{env, fs, fmt};

const DELIMITER: char = '|';
const SEPARATOR_LINE_NUM: usize = 1;
const FILENAME: &str = "input.txt";

#[derive(Clone, Copy, Debug)]
enum Alignment {
    Left,
    Right,
    Center,
}

#[derive(Debug)]
struct Column {
    width: usize,
    alignment: Alignment,
}

impl Default for Column {
    fn default() -> Self {
        Column {
            width: 0,
            alignment: Alignment::Left,
        }
    }
}

struct Table {
    content: Vec<String>,
    number_of_columns: usize,
    columns: Vec<Column>,
}

impl Table {
    fn new(lines: &[&str]) -> Self {
        let mut table = Table {
            content: lines.iter().map(|line| line.to_string()).collect(),
            number_of_columns: 0,
            columns: Vec::new(),
        };

        table.initialize_from_lines(lines);
        table
    }

    fn initialize_from_lines(&mut self, lines: &[&str]) {
        self.determine_number_of_columns(lines[SEPARATOR_LINE_NUM]);
        self.calculate_column_widths(lines);
        self.analyze_separator(lines[SEPARATOR_LINE_NUM]);
    }

    fn format_separation_line(&self) -> String {
        let formatted_line: Vec<String> = self
            .columns
            .iter()
            .map(|column| match column.alignment {
                Alignment::Left => format!(":{}-", "-".repeat(column.width)),
                Alignment::Right => format!("-{}:", "-".repeat(column.width)),
                Alignment::Center => format!(":{}:", "-".repeat(column.width)),
            })
            .collect();
        format!("|{}|", formatted_line.join("|"))
    }

    fn format_line(&self, line: &str) -> String {
        let columns = &self.split_line(line)[..self.number_of_columns];
        let formatted_line: Vec<String> = columns
            .iter()
            .enumerate()
            .map(|(i, column)| {
                let column_width = self.columns[i].width;
                let padding = " ".repeat(column_width.saturating_sub(column.len()));

                match self.columns[i].alignment {
                    Alignment::Left => format!("{}{}", column, padding),
                    Alignment::Right => format!("{}{}", padding, column),
                    Alignment::Center => {
                        let half_padding = padding.len() / 2;
                        format!(
                            "{}{}{}",
                            &padding[..half_padding],
                            column,
                            &padding[half_padding..]
                        )
                    }
                }
            })
            .collect();
        format!("| {} |", formatted_line.join(" | "))
    }

    fn split_line(&self, line: &str) -> Vec<String> {
        let mut previous_char_backslash: bool = false;
        let chars_iterable = line.chars().peekable();

        let mut cells: Vec<String> = Vec::new();
        let mut current_cell: String = String::new();

        for value in chars_iterable {
            if value == '|' && !previous_char_backslash {
                cells.push(current_cell.trim().to_string());
                current_cell = String::new();
            } else if previous_char_backslash {
                // current_cell.pop();
                current_cell.push(value);
            } else {
                current_cell.push(value);
            }

            previous_char_backslash = value == '\\';
        }

        let empty_cells = self.number_of_columns.saturating_sub(cells[1..].len());
        cells.extend(vec!["".to_string(); empty_cells]);
        cells[1..].to_vec()
    }
    /// Determines the number of columns in the GFM table
    /// Number of columns is determined by the separator row
    /// # Arguments
    /// * `line` - The header row
    fn determine_number_of_columns(&mut self, line: &str) {
        self.number_of_columns = line.split(DELIMITER).count() - 2;
    }

    /// Calculates the width of each column based on the content
    /// # Arguments
    /// * `lines` - The lines of the table
    fn calculate_column_widths(&mut self, lines: &[&str]) {
        for (i, line) in lines.iter().enumerate() {
            if i == 1 {
                continue;
            }
            let columns: &[String] = &self.split_line(line)[..self.number_of_columns];
        for (i, column) in columns.iter().enumerate() {
                let column_width: usize = column.len();
                if i >= self.columns.len() {
                    self.columns.push(Column {
                        width: column_width,
                        ..Default::default()
                    });
                }
                self.columns[i].width = self.columns[i].width.max(column_width);
            }
        }
    }
    /// Sets up the alignment of the columns based on the separator line
    /// # Arguments
    /// * `line` - The separator line
    fn analyze_separator(&mut self, line: &str) {
        let line = self.split_line(line);
        for (i, column) in line.iter().enumerate() {
            self.columns[i].alignment = match (column.starts_with(':'), column.ends_with(':')) {
                (true, true) => Alignment::Center,
                (true, false) => Alignment::Left,
                (false, true) => Alignment::Right,
                _ => Alignment::Left,
            };
        }
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        for (i, line) in self.content.iter().enumerate() {
            if i == SEPARATOR_LINE_NUM {
                output.push_str(&self.format_separation_line());
            } else {
                output.push_str(&self.format_line(line));
            }
            output.push('\n');
        }

        output.pop();

        write!(f, "{}", output)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename: &String = &args[1];

    let file_content = std::fs::read_to_string(filename).expect("Failed to read file");
    let lines: Vec<&str> = file_content.lines().collect();

    let table = Table::new(&lines);
    // table.tidy(&lines);
    println!("{}", table)
}

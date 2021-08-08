use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use colored::*;

pub struct QueryResult {
    line_link: String,
    line: String,
}

impl QueryResult {
    pub fn get_result(self) -> String {
        format!("{} {}", self.line_link, self.line)
    }
}

pub fn query_file(query: &str, filename: &str) -> Vec<QueryResult> {
    let mut result: Vec<QueryResult> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for (line_number, line) in lines.enumerate() {
            if let Ok(line_val) = line {
                if line_val.contains(query) {
                    let line_result = QueryResult {
                        line_link: format!("{}:{}", filename, line_number + 1),
                        line: line_val.replace(query, &query.red().bold().to_string()),
                    };
                    result.push(line_result);
                }
            }
        }
    };
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file_path = filename.as_ref().clone();
    let file = File::open(file_path);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!(
            "Failure while reading file - {}\nError: {}",
            filename.as_ref().to_str().unwrap(),
            error
        ),
    }
}

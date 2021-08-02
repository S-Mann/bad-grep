use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_args(&args);
    let result = query_file(query, filename);
    for val in result {
        println!("{} {}", val.line_link, val.line);
    }
}
struct QueryResult {
    line_link: String,
    line: String,
}

fn parse_args(args: &[String]) -> (&String, &String) {
    if args.len() >= 3 {
        (&args[1], &args[2])
    } else {
        panic!("Please pass valid arguments, query and filename")
    }
}

fn query_file(query: &String, filename: &String) -> Vec<QueryResult> {
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
    let file = File::open(filename);
    match file {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(error) => panic!("Failure while reading file: {}", error),
    }
}

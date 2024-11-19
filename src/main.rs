use std::io::{stdin, BufRead};
use std::process;
use boxes::find_boxes;

mod boxes;

fn main() {
    let lines = process_input().unwrap_or_else(|err| {
        eprintln!("Parsing Error: {err}");
        process::exit(1);
    });

    match find_boxes(&lines) {
        Ok(boxes) => {
            boxes.iter().for_each(|b| println!("{b}"));
        },
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
}

pub fn process_input() -> Result<Vec<String>, String> {
    let stdin = stdin();
    let handle = stdin.lock();
    let mut input_lines: Vec<String> = vec![];
    
    for line in handle.lines() {
        match line {
            Ok(text) => {
                if text.trim().is_empty() {
                    break;
                }
                input_lines.push(text);
            },
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }

    Ok(input_lines)
}
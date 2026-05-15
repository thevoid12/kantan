mod interpretor;
mod parser;
mod tokenizer;

use std::env;
use std::fs;
use std::process;

fn process_file(file_content: String) {
    let mut total_tokens = Vec::new();
    let mut is_long_notes = false;
    for lines in file_content.lines() {
        let trimmed_line = lines.trim();

        if trimmed_line.is_empty() {
            continue;
        }
        if is_long_notes {
            if trimmed_line == "}" {
                is_long_notes = false;
            }
            continue;
        }

        let tokens;
        (tokens, is_long_notes) = tokenizer::tokenize(trimmed_line.to_string(), is_long_notes);

        if !tokens.is_empty() {
            total_tokens.push(tokens);
        }
    }

    // println!("{:?}", &total_tokens);
    let ast = parser::parse(&total_tokens);
    // println!("{:?}", ast);
    interpretor::run(ast);
}

fn main() {
    let file = env::args().nth(1).expect("Please provide .kn file");
    let file_name_split = file.split(".");

    match file_name_split.last() {
        Some(file_extension) => {
            if file_extension != "kn" {
                eprintln!("Invalid file format, please provide file with .kn");
                process::exit(1);
            }

            if let Ok(content) = fs::read_to_string(&file) {
                process_file(content.to_string());
            } else {
                eprintln!("Cannot able to read the file: {}", &file);
            }
        }
        None => panic!("Invalid"),
    }
}

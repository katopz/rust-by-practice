use std::{env, fs};

use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, Write};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

enum ParseExpect {
    Number,
    CodeBegin,
    CodeEnd,
}

fn write_file_rs(
    quiz_file_name: &String,
    current_num_bullet: &String,
    sub_index: i32,
    rust_content: &String,
) {
    write!(
        File::create(&format!(
            "{}_{current_num_bullet}_{sub_index}.rs",
            quiz_file_name.to_lowercase().to_owned()
        ))
        .unwrap(),
        "{rust_content}"
    )
    .unwrap()
}

fn main() {
    let answer_file_name = "./solutions/basic-types/statements.md";
    let answer_file_names = answer_file_name.split("/").collect::<Vec<_>>();
    let quiz_file_name = format!(
        "./en/src/{}/{}",
        answer_file_names[2],
        ((answer_file_names[3])
            .to_string()
            .split_once(".")
            .unwrap()
            .0)
            .to_string()
    );

    let num_bullet_re = Regex::new(r"^[0-9]\.").unwrap();
    let code_begin_re = Regex::new(r"^```\w").unwrap();
    let code_end_re = Regex::new(r"^```\r?$").unwrap();

    let mut state = ParseExpect::Number;
    let mut rust_content = "".to_owned();

    let mut current_num_bullet = "".to_string();
    let mut current_sub_index = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(answer_file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            match line {
                Ok(text) => {
                    // Process state
                    match state {
                        ParseExpect::Number => {
                            // Begin with number and dot?
                            if num_bullet_re.is_match(&text.as_str()) {
                                current_num_bullet = text.split_once(".").unwrap().0.to_string();
                                state = ParseExpect::CodeBegin;
                                continue;
                            }

                            // Begin other code block?
                            if code_begin_re.is_match(text.as_str()) {
                                // Bump next sub filename index
                                current_sub_index = current_sub_index + 1;

                                // Clear
                                rust_content = "".to_owned();

                                // Next
                                state = ParseExpect::CodeEnd;
                                continue;
                            }
                        }
                        ParseExpect::CodeBegin => {
                            // Begin code block?
                            if code_begin_re.is_match(text.as_str()) {
                                state = ParseExpect::CodeEnd;

                                // New sub
                                current_sub_index = 0;

                                // Clear
                                rust_content = "".to_owned();
                                continue;
                            }
                        }
                        ParseExpect::CodeEnd => {
                            // Finish code block?
                            if code_end_re.is_match(text.as_str()) {
                                // Write current block
                                // println!("write:{rust_content:?}");
                                write_file_rs(
                                    &quiz_file_name,
                                    &current_num_bullet,
                                    current_sub_index,
                                    &rust_content,
                                );

                                // Clear
                                rust_content = "".to_owned();

                                // Next
                                state = ParseExpect::Number;
                                continue;
                            }

                            rust_content.push_str(text.as_str());
                            rust_content.push_str("\n".as_ref());
                        }
                    }
                }
                Err(_) => todo!(),
            }
        }
    }
}

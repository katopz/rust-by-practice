mod utils;
use std::{fs, process::Command};

use utils::{
    get_folders, get_md_files, get_rs_files, is_path_exists, read_lines, write_file_md,
    write_file_rs, ParseExpect, CODE_BEGIN_RE, CODE_END_RE, INSERTED_RS_RE, NUM_BULLET_RE,
};

fn generate_answer_rs(answer_file_name: &String) -> Result<(), anyhow::Error> {
    let answer_file_names = answer_file_name.split("/").collect::<Vec<_>>();
    let quiz_folder_name = format!("./en/src/{}", answer_file_names[2]);
    let quiz_file_name = format!(
        "{}/{}",
        quiz_folder_name,
        ((answer_file_names[3])
            .to_string()
            .split_once(".")
            .unwrap()
            .0)
            .to_string()
    );

    // Ensure md exist
    let quiz_file_name_md = &format!("{}.md", quiz_file_name);
    if !is_path_exists(quiz_file_name_md) {
        panic!("not found {quiz_file_name_md}");
    }

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
                            if NUM_BULLET_RE.is_match(text.as_str()) {
                                current_num_bullet = text.split_once(".").unwrap().0.to_string();
                                state = ParseExpect::CodeBegin;
                                continue;
                            }

                            // Begin other code block?
                            if CODE_BEGIN_RE.is_match(text.as_str()) {
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
                            if CODE_BEGIN_RE.is_match(text.as_str()) {
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
                            if CODE_END_RE.is_match(text.as_str()) {
                                // Write current block
                                write_file_rs(
                                    quiz_file_name.to_owned(),
                                    &current_num_bullet,
                                    current_sub_index,
                                    &rust_content,
                                )?;

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

    Ok(())
}

fn insert_answer_rs(answer_file_name: &String) -> Result<(), anyhow::Error> {
    let answer_file_names = answer_file_name.split("/").collect::<Vec<_>>();
    let quiz_folder_name = format!("./en/src/{}", answer_file_names[2]);
    let quiz_file_name = ((answer_file_names[3])
        .to_string()
        .split_once(".")
        .unwrap()
        .0)
        .to_string();
    let quiz_file_path = format!("{}/{}.md", quiz_folder_name, quiz_file_name);

    let rs_file_names = get_rs_files(&quiz_folder_name)?;

    let mut state = ParseExpect::Number;
    let mut rust_content = "".to_owned();

    let mut current_num_bullet = "".to_string();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&quiz_file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            match line {
                Ok(text) => {
                    // Keep all line, except inserted
                    if !INSERTED_RS_RE.is_match(text.as_str()) {
                        rust_content.push_str(text.as_str());
                        rust_content.push_str("\n".as_ref());
                    }

                    // Process state
                    match state {
                        ParseExpect::Number => {
                            // Begin with number and dot?
                            if NUM_BULLET_RE.is_match(text.as_str()) {
                                current_num_bullet = text.split_once(".").unwrap().0.to_string();
                                state = ParseExpect::CodeBegin;
                                continue;
                            }
                        }
                        ParseExpect::CodeBegin => {
                            // Begin code block?
                            if CODE_BEGIN_RE.is_match(text.as_str()) {
                                state = ParseExpect::CodeEnd;

                                continue;
                            }
                        }
                        ParseExpect::CodeEnd => {
                            // Finish code block?
                            if CODE_END_RE.is_match(text.as_str()) {
                                // Insert answer
                                let base_file_name =
                                    format!("{quiz_file_name}_{current_num_bullet}");

                                // Match answer(s)
                                rs_file_names
                                    .iter()
                                    .filter(|e| e.starts_with(&base_file_name))
                                    .for_each(|e| {
                                        // {{#playground statements_1_0.rs answer}}
                                        rust_content.push_str(
                                            format!("\n{{{{#playground {e} answer}}}}").as_str(),
                                        );
                                    });

                                if rs_file_names.len() > 0 {
                                    rust_content.push_str(format!("\n").as_str())
                                }

                                // Next
                                state = ParseExpect::Number;
                                continue;
                            }
                        }
                    }
                }
                Err(_) => todo!(),
            }
        }
    }

    Ok(write_file_md(&quiz_file_path, &rust_content)?)
}

fn generate_solution(path_string: &str) -> Result<(), anyhow::Error> {
    let folders = get_folders(&path_string.to_string())?;

    folders.iter().for_each(|folder| {
        let base_path = format!("{path_string}/{folder}");
        println!("process:{:?}", base_path);

        let md_file_names = get_md_files(&base_path.to_owned()).unwrap();

        // TODO: handle result
        let _results = md_file_names
            .iter()
            .map(|file_name| {
                let file_path = format!("{base_path}/{file_name}");
                // println!("file_path:{:?}", file_path);
                generate_answer_rs(&file_path).unwrap();
                insert_answer_rs(&file_path).unwrap();
            })
            .collect::<Vec<_>>();
    });

    Ok(())
}

fn format_md_rs(file_path: &String) -> Result<(), anyhow::Error> {
    let mut state = ParseExpect::Number;
    let mut md_content = "".to_owned();
    let mut rust_content = "".to_owned();

    let mut current_num_bullet = "".to_string();

    println!("parse format_md_rs: {file_path:#?}");

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(&file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            match line {
                Ok(text) => {
                    // Keep all content except code block
                    if state == ParseExpect::Number {
                        md_content.push_str(text.as_str());
                        md_content.push_str("\n".as_ref());
                    }

                    // Process state
                    match state {
                        ParseExpect::Number => {
                            // Begin with number and dot?
                            if NUM_BULLET_RE.is_match(text.as_str()) {
                                current_num_bullet = text.split_once(".").unwrap().0.to_string();
                                state = ParseExpect::CodeBegin;
                                continue;
                            }
                        }
                        ParseExpect::CodeBegin => {
                            // Keep
                            md_content.push_str(text.as_str());
                            md_content.push_str("\n".as_ref());

                            // Begin code block?
                            if CODE_BEGIN_RE.is_match(text.as_str()) {
                                state = ParseExpect::CodeEnd;

                                continue;
                            }
                        }
                        ParseExpect::CodeEnd => {
                            // Finish code block?
                            if CODE_END_RE.is_match(text.as_str()) {
                                // Format
                                let temp_file_path = "temp.md".to_owned();
                                write_file_md(&temp_file_path, &rust_content)?;
                                let mut format_cmd = Command::new("rustfmt");
                                format_cmd.arg("-v").arg(&temp_file_path);
                                let format_output =
                                    format_cmd.output().expect("failed to execute format");
                                if format_output.stderr.len() > 0 {
                                    println!("{:?}", format_output.stderr)
                                }
                                let formatted_rust_content = fs::read_to_string(&temp_file_path)
                                    .expect("Should have been able to read the file");
                                fs::remove_file(temp_file_path)?;

                                // Merge code
                                md_content.push_str(formatted_rust_content.as_str());
                                md_content.push_str(text.as_str());
                                md_content.push_str("\n".as_ref());

                                println!("formatted: {current_num_bullet:?}");

                                // Next
                                state = ParseExpect::Number;
                                rust_content = "".to_owned();
                                continue;
                            }

                            // Keep code
                            rust_content.push_str(text.as_str());
                            rust_content.push_str("\n".as_ref());
                        }
                    }
                }
                Err(_) => todo!(),
            }
        }
    }

    Ok(write_file_md(&format!("{file_path}"), &md_content)?)
}

fn format_md_rs_in_folder(path_string: &str) -> Result<(), anyhow::Error> {
    let folders = get_folders(&path_string.to_string())?;

    // TODO : DRY
    folders.iter().for_each(|folder| {
        let base_path = format!("{path_string}/{folder}");
        let md_file_names = get_md_files(&base_path.to_owned()).unwrap();

        // TODO: handle result
        let _results = md_file_names
            .iter()
            .map(|file_name| {
                let file_path = format!("{base_path}/{file_name}");
                println!("format:{:?}", file_path);
                format_md_rs(&file_path).unwrap();
            })
            .collect::<Vec<_>>();
    });

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    format_md_rs(&"./solutions/compound-types/struct.md".to_string())
    // format_md_rs_in_folder("./solutions")
    // generate_solution("./solutions")
}

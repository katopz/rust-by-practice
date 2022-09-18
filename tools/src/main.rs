mod utils;

use utils::{
    is_path_exists, list_md_file, list_rs_file, read_lines, write_file_md, write_file_rs,
    ParseExpect, CODE_BEGIN_RE, CODE_END_RE, INSERTED_RS_RE, NUM_BULLET_RE,
};

pub fn generate_answer_rs(answer_file_name: &String) {
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

pub fn insert_answer_rs(answer_file_name: &String) {
    let answer_file_names = answer_file_name.split("/").collect::<Vec<_>>();
    let quiz_folder_name = format!("./en/src/{}", answer_file_names[2]);
    let quiz_file_name = ((answer_file_names[3])
        .to_string()
        .split_once(".")
        .unwrap()
        .0)
        .to_string();
    let quiz_file_path = format!("{}/{}.md", quiz_folder_name, quiz_file_name);

    let rs_file_names = list_rs_file(&quiz_folder_name);

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
                                rust_content.push_str("\n".as_ref());

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

    write_file_md(&quiz_file_path, &rust_content)
}

fn main() {
    // Loop add md in folder
    let base_path = "./solutions/basic-types";
    let md_file_names = list_md_file(&base_path.to_owned());

    // TODO: handle result
    let _results = md_file_names
        .iter()
        .map(|file_name| {
            let file_path = format!("{base_path}/{file_name}");
            // println!("file_path:{:?}", file_path);
            generate_answer_rs(&file_path);
            insert_answer_rs(&file_path);
        })
        .collect::<Vec<_>>();
}

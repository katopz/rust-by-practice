use std::{env, fs};

fn parse_file(file_path: String) {
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path.clone())
        .expect(format!("{:?}/{:?}", env::current_dir().unwrap(), file_path).as_str());

    let foo = contents
        .split("\n\n")
        .into_iter()
        .map(|e| e)
        .collect::<Vec<_>>();
    println!("{:#?}", foo);
}

fn main() {
    println!("{:?}", env::current_dir());
    // ./solutions/compound-types/slice.md
    // ./solutions/basic-types/statements.md
    let file_path = "./solutions/basic-types/statements.md".to_string();
    parse_file(file_path);
}

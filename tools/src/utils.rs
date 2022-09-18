use lazy_static::lazy_static;
use regex::Regex;

use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

lazy_static! {
    pub(crate) static ref NUM_BULLET_RE: Regex = Regex::new(r"^[0-9]\.").unwrap();
    pub(crate) static ref CODE_BEGIN_RE: Regex = Regex::new(r"^```\w").unwrap();
    pub(crate) static ref CODE_END_RE: Regex = Regex::new(r"^```\r?$").unwrap();
    pub(crate) static ref INSERTED_RS_RE: Regex =
        Regex::new(r"^(\{\{#playground\s)(\w+\.rs\s)(answer\}\})$").unwrap();
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub(crate) fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub(crate) enum ParseExpect {
    Number,
    CodeBegin,
    CodeEnd,
}

pub(crate) fn write_file_rs(
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

pub(crate) fn write_file_md(file_path: &String, rust_content: &String) {
    write!(
        File::create(file_path.to_lowercase().to_owned()).unwrap(),
        "{rust_content}"
    )
    .unwrap()
}

pub(crate) fn get_filtered_folders(
    dir: &Path,
    pattern_re: &Regex,
) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|path| path.is_file())
        .filter(|path| pattern_re.is_match(path.file_name().unwrap().to_str().unwrap())) // Filter out non-file
        .collect())
}

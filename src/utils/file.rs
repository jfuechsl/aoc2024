use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn load_file_lines(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

pub fn read_lines<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf_reader = io::BufReader::new(file);
    let mut contents = String::new();

    for line in buf_reader.lines() {
        contents.push_str(&line?);
        contents.push('\n');
    }

    Ok(contents)
}

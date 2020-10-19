use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_txt(filepath: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut lines = vec![];
    for line in BufReader::new(File::open(filepath)?).lines() {
        lines.push(line?);
    }
    Ok(lines)
}
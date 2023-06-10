use std::fs;
mod lexer;
mod util;

const FILEPATH: &str = "input_files/test.basm";

fn main() {
    let data: String = read_file_string(FILEPATH).unwrap();
    let data: Vec<Vec<String>> = string_to_array(data);
    println!("{:?}", data);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn string_to_array(file: String) -> Vec<Vec<String>> {
    let mut data_copy = file.split("\n").collect::<Vec<&str>>();
    let mut lexer = lexer::Lexer::new();

    while data_copy.len() > 0 {
        let line = util::strip_whitespace(data_copy[0].to_string());
        data_copy.remove(0);
        if line.len() == 0 { continue; }
        lexer.translate_to_asm(line);
    }

    return vec![lexer.data, lexer.code];
}
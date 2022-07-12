use std::fs;
use std::str;
use regex::Regex;
mod tokens;
mod parser;

fn main() {
  let contents = fs::read_to_string("./src/test.wat")
        .expect("Something went wrong reading the file");
  
    let oneline: String = str::replace(&contents, "\n", "");
    let re = Regex::new(r"\s+").unwrap();
    let text = re.replace_all(&oneline, " ");
    let chars = text.trim();
    let parsed: Vec<parser::Token> = parser::Parser::new(chars).collect();
    println!("THE PARSED TOKENS {:?}", parsed);
}

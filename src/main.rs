use std::fs;
use std::str;
use regex::Regex;
mod tokens;
mod parser;
mod ast;

fn main() {
  let contents = fs::read_to_string("./src/test.wat")
        .expect("Something went wrong reading the file");
  
    let oneline: String = str::replace(&contents, "\n", "");
    let re = Regex::new(r"\s+").unwrap();
    let text = re.replace_all(&oneline, " ");
    let chars = text.trim();
    let parsed = parser::Parser::new(chars);
    let ast = ast::ast_builder(parsed);
    println!("THE AST {:?}", ast);
}

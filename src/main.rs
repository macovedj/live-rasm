use std::fs;
use std::str;
use std::io::prelude::*;
use regex::Regex;
mod tokens;
mod parser;
mod ast;
mod compiler;

fn main() -> std::io::Result<()> {
  let contents = fs::read_to_string("./src/test.wat")
        .expect("Something went wrong reading the file");
  
  let oneline: String = str::replace(&contents, "\n", "");
  let re = Regex::new(r"\s+").unwrap();
  let text = re.replace_all(&oneline, " ");
  let chars = text.trim();
  let parsed = parser::Parser::new(chars);
  let ast = ast::ast_builder(parsed);
  let bytes = compiler::compiler(ast);
  let hex = bytes.iter().map(|x| format!("{:x}", x)).collect::<Vec<_>>().join("");
  let mut file = fs::File::create("add4.wasm").unwrap();
  file.write(&bytes);
  println!("THE HEX {:?}", hex);
  Ok(())
}
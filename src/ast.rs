use super::parser;
use super::tokens;

#[derive(Debug)]
pub struct Ast<'a> {
  pub module: Vec<Func<'a>>
}

#[derive(Debug)]
pub struct Func<'a> {
  pub export: &'a str,
  pub params: Vec<WasmPrimitives>,
  pub body: Vec<&'a str>,
  pub result: WasmPrimitives
}

#[derive(Debug)]
pub enum WasmPrimitives {
  i32,
  i64,
  f32,
  f64,
  NULL
}

pub fn ast_builder<'a>(mut tokens: impl Iterator<Item = &'a parser::Token<'a>>) -> Ast<'a> {
  let mut ast = Ast {
    module: Vec::new()
  };

  while let Some(mut token) = tokens.next() {
    println!("THE TOKEN {:?}", token);
    match token.kind {
      tokens::TokenTypes::LPAR => {
        continue;
      }
      tokens::TokenTypes::RPAR => {
        continue;
      }
      tokens::TokenTypes::MOD => {
        continue;
      }
      tokens::TokenTypes::FUNC => {
        let mut func_def = true;
        let mut cur_func = Func {
          export: "",
          params: Vec::new(),
          body: Vec::new(),
          result: WasmPrimitives::NULL
        };
        tokens.next();
        if let Some(potential_token) = tokens.next() {
          token = potential_token;
        }
        while func_def {
          println!("THE CURRENT FUNCTION {:?}\n", cur_func);
          match token.kind {
            tokens::TokenTypes::EXPORT => {
              if let Some(next_token) = tokens.next() {
                let export = &next_token.value;
                cur_func.export = export;
                if let Some(potential_token) = tokens.next() {
                  token = potential_token;
                }
              }
              println!("THE CURRENT FUNCTION {:?}\n", cur_func);
            }
            tokens::TokenTypes::RPAR => {
              if let Some(potential_token) = tokens.next() {
                token = potential_token;
              }
              continue;
            }
            tokens::TokenTypes::LPAR => {
              if let Some(potential_token) = tokens.next() {
                token = potential_token;
              }
              continue;
            }
            tokens::TokenTypes::PARAMDECL => {
              if let Some(mut next_token) = tokens.next() {
                while matches!(next_token.kind, tokens::TokenTypes::PARAM) {
                  if next_token.value == "i32" {
                    cur_func.params.push(WasmPrimitives::i32);
                  }
                  if let Some(potential_token) = tokens.next() {
                    next_token = potential_token;
                  }
                  continue;
                }
                if let Some(potential_token) = tokens.next() {
                  token = potential_token;
                }
                continue;
              }
            }
            tokens::TokenTypes::RESULT => {
              cur_func.result = WasmPrimitives::i32;
              tokens.next();
              tokens.next();
              if let Some(potential_token) = tokens.next() {
                token = potential_token;
              }
              continue;
            }
            _ => {
              while !matches!(token.kind, tokens::TokenTypes::RPAR) {
                let instr = &token.value;
                cur_func.body.push(instr);
                if let Some(potential_token) = tokens.next() {
                  token = potential_token;
                }
              }
              tokens.next();
              func_def = false;
              ast.module.push(cur_func);
              break;
            }
          }
        }
      }
      _ => {}
    }
  }
  ast
}
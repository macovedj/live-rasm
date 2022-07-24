use super::parser;
use super::tokens;

#[derive(Debug)]
pub struct Ast {
  pub module: Vec<Func>
}

#[derive(Debug)]
pub struct Func {
  pub export: String,
  pub params: Vec<WasmPrimitives>,
  pub body: Vec<String>,
  pub result: WasmPrimitives
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum WasmPrimitives {
  i32,
  i64,
  f32,
  f64,
  NULL
}

pub fn ast_builder(mut tokens: impl Iterator<Item = parser::Token>) -> Ast {
  let mut ast = Ast {
    module: Vec::new()
  };

  while let Some(mut token) = tokens.next() {
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
          export: String::from(""),
          params: Vec::new(),
          body: Vec::new(),
          result: WasmPrimitives::NULL
        };
        tokens.next();
        if let Some(potential_token) = tokens.next() {
          token = potential_token;
        }
        while func_def {
          match token.kind {
            tokens::TokenTypes::EXPORT => {
              if let Some(next_token) = tokens.next() {
                let export = &next_token.value;
                cur_func.export = String::from(export);
                if let Some(potential_token) = tokens.next() {
                  token = potential_token;
                }
              }
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
                cur_func.body.push(String::from(instr));
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
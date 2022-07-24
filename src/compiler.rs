use core::num;

use super::ast;

#[derive(PartialEq, Eq)]
struct Sig <'a> {
  params: &'a [ast::WasmPrimitives],
  result: ast::WasmPrimitives
}

impl <'a> Sig <'a> {
  pub fn new(func: &'a ast::Func) -> Self {
    Sig { params: &func.params[..], result: func.result}
  }

  pub fn write(&self, bytes: &mut Vec<u8>) {
    bytes.push(0x60);
    bytes.push(self.params.len() as u8);
    for param in self.params {
      match param {
        ast::WasmPrimitives::i32 => {
          bytes.push(0x7f);
        }
        _ => {
          continue;
        }
      }
    }
    bytes.push(0x01);
    match self.result {
      ast::WasmPrimitives::i32 => {
        bytes.push(0x7f);
      }
      _ => {}
    }
  }
}

pub fn compiler(ast: ast::Ast) -> Vec<u8> {
  let mut bytes = Vec::new();
  bytes.push(0x00);
  bytes.push(0x61);
  bytes.push(0x73);
  bytes.push(0x6d);
  bytes.push(0x01);
  bytes.push(0x00);
  bytes.push(0x00);
  bytes.push(0x00);
  bytes.push(0x01);

  let mut sigs = Vec::new();
  for func in &ast.module[..] {
    sigs.push(Sig::new(&func));
  }
  sigs.dedup();
  let num_of_types = sigs.len();
  let type_section_size = &sigs[..].iter()
    .fold(0, |acc, cur| acc + 4 + cur.params.len()) + 1;
  bytes.push(type_section_size as u8);
  bytes.push(num_of_types as u8);
  for sig in &sigs[..] {
    sig.write(&mut bytes);
  }

  bytes.push(0x03);
  let num_of_funcs = ast.module.len();
  bytes.push((num_of_funcs + 1) as u8);
  bytes.push(num_of_funcs as u8);
  for func in &ast.module[..] {
    for (i, sig) in sigs.iter().enumerate() {
      if sig == &Sig::new(&func) {
        bytes.push(i as u8);
      }
    }
  }

  let num_of_exports = ast.module.iter()
    .filter(|func| func.export.len() > 0).count();

  if num_of_exports > 0 {
    bytes.push(0x07);
    let export_names = ast.module.iter().map(|f| &f.export).collect::<Vec<_>>();
    let export_section_size = ast.module.iter().map(|f| &f.export)
      .fold(0, |acc, cur| acc + cur.len() + 3);
    bytes.push((export_section_size + 1) as u8);
    bytes.push(num_of_exports as u8);
    for (i, export) in export_names.iter().enumerate() {
      if export.len() == 0 {
        continue;
      }
      bytes.push(export.len() as u8);
      for char in export.bytes() {
        if char >= 97 {
          bytes.push(char.to_ascii_lowercase())
        } else {
          bytes.push(char.to_ascii_uppercase())
        }
      }
      bytes.push(0x00);
      bytes.push(i as u8);
    }
  }
  bytes.push(0x0a);
  let body_section_length = ast.module.iter().map(|f| &f.body)
    .fold(0, |all_length, cur_body| all_length + cur_body.len() + 3) + 1;
  
  bytes.push(body_section_length as u8);
  bytes.push(ast.module.len() as u8);

  let bodies = ast.module.iter().map(|f| &f.body).collect::<Vec<_>>();
  for body in bodies {
    bytes.push((body.len() + 2) as u8);
    bytes.push(0x00);
    for instr in body {
      match instr.as_str() {
        "local.get" => {
          bytes.push(0x20);
        }
        "i32.add" => {
          bytes.push(0x6a);
        }
        _ => {
          for byte in instr.chars() {
            bytes.push(String::from(byte).parse::<u8>().unwrap());
          }
        }
      }
    }
    bytes.push(0x0b);
  }
  bytes
}
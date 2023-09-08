#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn get_json_cst(text: String) -> String {
  let mut parser = tree_sitter::Parser::new();
  parser.set_language(tree_sitter_json::language()).unwrap();
  let tree = parser.parse(text, None).unwrap();

  format!("{:?}", tree)
}

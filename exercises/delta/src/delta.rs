// Scissored from https://dev.to/h_ajsf/rust--wasm-using-bindgen-49b4
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(my_text :&str) {
  let mut s = my_text.chars();
  let d = s.sort().collect().to_string();

  return d;
}

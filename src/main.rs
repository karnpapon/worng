pub mod yuth;
// pub mod bin; // not part of interpreter, only for debugging.

use std::env;
use yuth::yuth::Yuth;
use yuth::expr;
use yuth::ast_printer;
use yuth::parser;
use yuth::interpreter;
// use tool::GenerateAst; 


fn main() {
  let args: Vec<String> = env::args().collect();
  let v: Vec<&str> = args.iter().map(|x| &**x).collect(); // Vec<String> -> Vec<&str>
  let mut l = Yuth { 
    had_error: false, 
    had_runtime_error: false, 
  };

  Yuth::main(&mut l, v);

}

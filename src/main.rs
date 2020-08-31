extern crate worng;
// pub mod bin; // not part of interpreter, only for debugging.

use std::env;
use worng::Worng;

fn main() {
  // let args: Vec<String> = env::args().collect();
  let mut args = env::args();
  args.next();
  let args: Vec<String> = args.collect();

  let v: Vec<&str> = args.iter().map(|x| &**x).collect(); // Vec<String> -> Vec<&str>
  let mut l = Worng { 
    had_error: false, 
    had_runtime_error: false, 
  };

  Worng::main(&mut l, v);

}

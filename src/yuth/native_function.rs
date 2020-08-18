use std::time::{SystemTime, UNIX_EPOCH};
use std::any::Any;


use super::callable::Callable;
use super::yuth::YuthValue;
use super::error::RuntimeError;
use super::interpreter::Interpreter;

pub fn get_current_time() -> u64 {
  SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs()
}

#[derive(Debug)]
pub struct NativeClock {}

impl NativeClock {
  pub fn new() -> NativeClock {
    NativeClock{}
  }
}

impl Callable for NativeClock {
  fn arity(&self) -> usize {
    return 0;
  }

  fn call(&self, interpreter: &mut Interpreter, args: Vec<YuthValue> ) ->  Result<YuthValue, RuntimeError>{
    Ok(YuthValue::Number(get_current_time() as f64)) 
  }

  fn func_to_string(&self) -> String{
    String::from("<native function>")
  }

  fn as_any(&self) -> &dyn Any{
    self
  }
}




use super::yuth::YuthValue;
use super::error::RuntimeError;
use super::interpreter::Interpreter;

pub trait Callable: std::fmt::Debug {
  fn call(&self, interpreter: &mut Interpreter, args: Vec<YuthValue>) -> Result<YuthValue, RuntimeError>;
  fn arity(&self) -> usize;
  fn func_to_string(&self) -> String;
}



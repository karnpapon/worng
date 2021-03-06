use super::worng_value::WorngValue;
use super::error::RuntimeError;
use super::interpreter::Interpreter;
use std::any::Any;

pub trait Callable: std::fmt::Debug {
  fn call(&self, interpreter: &mut Interpreter, args: Vec<WorngValue>) -> Result<WorngValue, RuntimeError>;
  fn arity(&self) -> usize;
  fn func_to_string(&self) -> String;
  fn as_any(&self) -> &dyn Any; 
}



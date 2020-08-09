use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::callable::Callable;
use super::interpreter::Interpreter;
use super::error::{ RuntimeError, YuthError, ValueError };
use super::yuth::YuthValue;
use super::token::Token;
use super::yuth_function::YuthFunction;
use super::yuth_instance::YuthInstance;

#[derive(Debug, Clone)]
pub struct YuthClass {
  name: String,
  methods: HashMap<String, YuthFunction>
}

impl YuthClass {
  pub fn new(name: String, methods: HashMap<String, YuthFunction>) -> YuthClass {
    YuthClass {
      name: name,
      methods: methods
    }
  }

  pub fn find_method(&self, name: Token) -> Result<YuthFunction, RuntimeError> {
    match self.methods.get(&name.lexeme) {
      Some(method) => Ok(method.clone()),
      None => Err(RuntimeError::UndefinedVariable(name))
    }
  }
}

impl std::fmt::Display for YuthClass {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Callable for YuthClass {

  fn call(
    &self, 
    interpreter: &mut Interpreter, 
    args: Vec<YuthValue>) 
    -> Result<YuthValue, RuntimeError>{

      let instance = YuthInstance::new(self.clone());
      return Ok(YuthValue::Instance(Rc::new( RefCell::new( instance )) ));
  }

  fn arity(&self)  -> usize{
    return 0
  }
  fn func_to_string(&self) -> String {
    return String::from("class func_to_string")
  }
}
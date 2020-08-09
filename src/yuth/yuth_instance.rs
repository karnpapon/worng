use std::collections::{HashMap};
use std::rc::Rc;

use super::expr::Expr;
use super::yuth_class::YuthClass;
use super::yuth::YuthValue;
use super::token::{Literal, Token};
use super::error::RuntimeError;

#[derive(Debug, Clone)]
pub struct YuthInstance {
  klass: YuthClass,
  fields: HashMap<String, YuthValue>
}

impl YuthInstance {
  pub fn new(klass: YuthClass) -> YuthInstance {
    YuthInstance {
      klass: klass,
      fields: HashMap::new()
    }
  }

  pub fn get(&self, name: Token ) -> Result<YuthValue, RuntimeError>{
    if let Some(value) =  self.fields.get(&name.lexeme) {
      return Ok(value.clone());
    }

    match self.klass.find_method(name){
      Ok(method) => Ok(YuthValue::Func(Rc::new(method))),
      Err(e) => Err(e)
    }
  }

  pub fn set(&mut self, name: Token, value: YuthValue) {
    self.fields.insert(name.lexeme, value);
  }
}

impl std::fmt::Display for YuthInstance {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.klass)
  }
}
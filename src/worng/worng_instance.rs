use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use super::expr::Expr;
use super::worng_class::WorngClass;
use super::worng_value::WorngValue;
use super::token::{Literal, Token};
use super::error::{ RuntimeError };

#[derive(Debug, Clone)]
pub struct WorngInstance {
  klass: WorngClass,
  fields: HashMap<String, WorngValue>
}

impl WorngInstance {
  pub fn new(klass: WorngClass) -> WorngInstance {
    WorngInstance {
      klass: klass,
      fields: HashMap::new()
    }
  }

  pub fn get(&self, name: &Token ) -> Result<WorngValue, RuntimeError>{
    if let Some(value) =  self.fields.get(&name.lexeme) {
      return Ok(value.clone());
    }

    match self.klass.find_method(&name.lexeme, Rc::new(RefCell::new(self.clone()))){
      Some(method) => Ok(WorngValue::Func(Rc::new(method))),
      None => Err(RuntimeError::UndefinedProperty(name.clone()))
    }
  }

  pub fn set(&mut self, name: Token, value: WorngValue) {
    self.fields.insert(name.lexeme, value);
  }
}

impl std::fmt::Display for WorngInstance {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.klass)
  }
}
use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use super::expr::Expr;
use super::token::{ Token };
use super::error::{ RuntimeError, EnvironmentError };
use super::yuth::{YuthValue};

#[derive(Debug)]
pub struct Environment {
  pub enclosing: Option<Rc<RefCell<Environment>>>,
  values: HashMap<String, YuthValue>
}

impl Environment {
  pub fn new() -> Self {
    Environment{
      enclosing: None,
      values: HashMap::new()
    }
  }

  pub fn define(&mut self, string: String, value: YuthValue) {
    self.values.insert(string, value); 
  }

  pub fn enclose(parent: Rc<RefCell<Environment>>) -> Environment {
    Environment {
      values: HashMap::new(),
      enclosing: Some(parent),
    }
  }

  pub fn get_value(&self, name: &Token) -> Result< YuthValue , EnvironmentError> {

    if let Some(val) = self.values.get(&name.lexeme){
      return Ok(val.clone());
    }

    if let Some(enclosing) = &self.enclosing {
      return enclosing.borrow().get_value(name);
    }

    Err(EnvironmentError::EnvironmentError) // no variable found.
  }

  pub fn assign(&mut self, key: &String, value: YuthValue) -> Result<() , EnvironmentError>  {
    if self.values.contains_key(key) {
      self.values.insert(key.clone(), value);
      return Ok(())
    } 

    if let Some(enclosing) = &self.enclosing {
      return enclosing.borrow_mut().assign(key, value);
    }

    Err(EnvironmentError::UndefinedVariable(key.clone()))
  }
}
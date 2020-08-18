use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;

use super::expr::Expr;
use super::native_function::NativeClock;
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

  pub fn global() -> Environment {
    let mut env = Environment::new();
    env.define( "clock".to_string(), YuthValue::Func(Rc::new(NativeClock::new())));
    env
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

  pub fn get_at(&self, distance: usize, key: &Token) -> Result<YuthValue, EnvironmentError> {

    if distance == 0 {
      return self.get_value(key);
    }

    let val = match self.ancestor(distance){
      Some(enclosed_scope) => { enclosed_scope.borrow().get_value(key) },
      None => Err(EnvironmentError::UndefinedVariable(key.lexeme.clone())),
    };
    val
  }

  fn ancestor(&self, distance: usize) -> Option<Rc<RefCell<Environment>>> {
    let mut ancestor = match self.enclosing {
      Some(ref enclosed_env) => enclosed_env.clone(),
      None => return None,
    };

    // stuck in this for awhilee (change from 0..distance)
    for _ in 1..distance {
      let new_env = match ancestor.borrow().enclosing {
        Some(ref parent_env) => parent_env.clone(),
        None => return None,
    };
      ancestor = new_env;
    }
    Some(ancestor)
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

  pub fn assign_at(&mut self, distance: usize, key: &Token, value: YuthValue) -> Result<(), EnvironmentError> {
    if distance == 0 {
      return self.assign(&key.lexeme, value);
    }

    match self.ancestor(distance){
      Some(ref mut enclosed_env) => enclosed_env.borrow_mut().assign(&key.lexeme, value) ,
      None => Err(EnvironmentError::UndefinedVariable(key.lexeme.clone())),
    }
  }
}
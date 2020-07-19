use std::io::{ self, Read, Write};
use std::fs;
use std::path::Path;
use std::error::{Error};
use std::ops::Sub;
use std::rc::Rc;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use super::scanner::Scanner;
use super::statement::Stmt;
use super::token::Token;
use super::token_type::TokenType;
use super::callable::Callable;
use super::parser::Parser;
use super::interpreter::Interpreter;
use super::expr::Expr;
use super::error::{ YuthError, ValueError, RuntimeError };

#[derive(Debug)]
pub struct Yuth {
  pub had_error: bool,
  pub had_runtime_error: bool,
}

#[derive(Debug)]
pub enum YuthValue {
  // Identifier(String),
  Number(f64),
  String(String),
  Bool(bool),
  Func(Rc<dyn Callable>),
  Nil,
}

// pub enum YuthError{
//   InternalError(String),
// }

impl std::clone::Clone for YuthValue {
  fn clone(&self) -> YuthValue {
      match *self {
          YuthValue::Number(number) => YuthValue::Number(number),
          YuthValue::String(ref string) => YuthValue::String(string.clone()),
          YuthValue::Bool(b) => YuthValue::Bool(b),
          YuthValue::Nil => YuthValue::Nil,
          YuthValue::Func(ref func) => YuthValue::Func(func.clone()),
      }
  }
}


impl Yuth {
  
  pub fn main(&mut self, args: Vec<&str> ) {
    
    if args.len() > 2 {

      writeln!(io::stdout(), "Usage: yuth [script]");
      std::process::exit(64)
    } else if args.len() == 2 {
      self.run_file(String::from(args[1]));
    } else {
      self.run_prompt();
    }
  }
  
  fn run_file(&mut self, path: String) -> Result<(), RuntimeError> {
    let _path = Path::new(&path);
    let mut bytes = match fs::read_to_string(_path){
      Err(err) => panic!("couldn't open file: {}", err),
      Ok(file) => file
    };
  
    self.run(String::from(bytes));
     // Indicate an error in the exit code.
    if self.had_error { std::process::exit(64); }
    if self.had_runtime_error { std::process::exit(70); }
    Ok(())
  }
  
  fn run_prompt(&mut self) -> Result<(), Box<Error>>{
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
      println!("No previous history.");
    }
  
    loop {
      let readline = rl.readline(">> ");
      let mut l = match readline {
        Ok(line) => { rl.add_history_entry(line.as_str()); line },
        Err(ReadlineError::Interrupted) => { println!("exit"); break },
        Err(err) => { println!("Error: {:?}", err); break }
      };
      self.run(l);
    }
  
    rl.save_history("history.txt").unwrap();
  
    Ok(())
  }
  
  fn run(&mut self, source: String) {
    let mut scanner = Scanner::new(&source);
    let tokens: Vec<Token> = scanner.scan_tokens();
    
    let mut parser = Parser::new(tokens);
    let expression: Vec<Stmt> = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();

    if self.had_error { return };

    interpreter.interpret(expression);
  }

  fn report(&mut self, line: i32, pos: String, message: &str) {
    eprintln!("[Line {}] Error {}: {}", line,pos,message);
    self.had_error = true;
  }
  
  pub fn error(&mut self, token: Token, message: &str) {
    if token.token_type == TokenType::EOF {
      self.report(token.line, String::from(" at end"), message);
    } else {
      self.report(token.line, String::from(format!(" at '{}'",token.lexeme )), message);
    }
  }
}

impl YuthValue {
  pub fn subtract(&self, other: YuthValue) -> Result<YuthValue, ValueError> {
    match (self, other) {
      (&YuthValue::Number(left), YuthValue::Number(right)) => {
        Ok(YuthValue::Number(left - right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn add(&self, other: YuthValue) -> Result<YuthValue, ValueError>{
    match(self, other){
      (&YuthValue::Number(left), YuthValue::Number(right)) => {
        Ok(YuthValue::Number(left + right))
      },
      // NOTE: should be possible to concat string with number.
      (&YuthValue::String(ref left), YuthValue::String(ref right)) => {
        let mut s = String::from(left);
        s.push_str(right);
        Ok(YuthValue::String(s))
      },
      _ => Err(ValueError::TypeError) 
    }
  }

  pub fn divide(&self, other: YuthValue) -> Result<YuthValue, YuthError>{
    match(self, other) {
      (&YuthValue::Number(left), YuthValue::Number(0.0)) => {
        Err(YuthError::RuntimeError(RuntimeError::RuntimeError))
      },
      (&YuthValue::Number(left), YuthValue::Number(right)) => {
        Ok(YuthValue::Number(left / right))
      },
      _ => Err(YuthError::RuntimeError(RuntimeError::RuntimeError) )
    }
  }

  pub fn multiply(&self, other: YuthValue) -> Result<YuthValue, ValueError>{
    match(self, other){
      (&YuthValue::Number(left), YuthValue::Number(right)) => {
        Ok(YuthValue::Number(left * right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn greater(&self, other: YuthValue) -> Result<YuthValue, ValueError> {
    match(self, other){
      (&YuthValue::Number(left), YuthValue::Number(right)) => {
        Ok(YuthValue::Bool(left > right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn greater_than(&self, other: YuthValue) -> Result<YuthValue, ValueError> {
    match(self, other){
      (&YuthValue::Number(left), YuthValue::Number(right)) => {
        Ok(YuthValue::Bool(left > right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn greater_equal(&self, other: YuthValue) -> Result<YuthValue, ValueError>{
    match(self, other){
      (&YuthValue::Number(ref left), YuthValue::Number(ref right) ) => {
        Ok(YuthValue::Bool( left >= right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn less_than(&self, other: YuthValue) -> Result<YuthValue, ValueError>{
    match(&self, &other){
      (YuthValue::Number(left ), YuthValue::Number(right)) => {
        Ok(YuthValue::Bool( left < right ))
      },
     _ => Err(ValueError::TypeError)
    }
  }

  pub fn less_equal(&self, other: YuthValue) -> Result<YuthValue, ValueError> {
    match(&self, &other) {
      (YuthValue::Number(left), YuthValue::Number(right)) => {
      Ok(YuthValue::Bool(left <= right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn bang_equal(&self, other: YuthValue) -> Result<YuthValue, ValueError> {
    Ok(YuthValue::Bool( !self.is_equal(&other)))
  }

  pub fn equal_equal(&self, other: YuthValue) -> Result<YuthValue, ValueError>{
    Ok(YuthValue::Bool( self.is_equal(&other)))
  }

  pub fn is_equal(&self, other: &YuthValue) -> bool {
    let boo = match (&self , &other) {
      (&YuthValue::Nil, &YuthValue::Nil) => { true},
      (&YuthValue::Bool(b), &YuthValue::Bool(other)) => b == other,
      (&YuthValue::String(ref string), &YuthValue::String(ref other)) => string == other,
      (&YuthValue::Number(num), &YuthValue::Number(other)) => num == other,
      (&YuthValue::Func(ref f), &YuthValue::Func(ref other)) => Rc::ptr_eq(f, other),
      _ => false,
    };

    boo
  }

  pub fn negate_number(&self) -> Result<YuthValue, ValueError> {
    if let YuthValue::Number(ref num) = *self {
      Ok(YuthValue::Number(-num))
    } else {
      Err(ValueError::TypeError)
    }
  }

  pub fn negate(&self) -> Result<YuthValue, ValueError> {
    Ok(YuthValue::Bool(!self.is_truthy())) 
  }

  pub fn is_truthy(&self) -> bool {
    match *self {
      YuthValue::Nil => false,
      YuthValue::Bool(b) => b,
      _ => true,
    }
  }

  pub fn get_callable(&self) -> Option<Rc<dyn Callable>> {
    match *self {
      YuthValue::Func(ref func) => Some(func.clone()),
      _ => None,
    }
  }

}


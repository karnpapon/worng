extern crate rustyline;

use std::io::{ self, Read, Write};
use std::fs;
use std::path::Path;
use std::error::{Error};
use std::ops::Sub;
use std::io::Cursor;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;

use self::rustyline::error::ReadlineError;
use self::rustyline::Editor;

use super::scanner::Scanner;
use super::statement::Stmt;
use super::token::Token;
use super::token_type::TokenType;
use super::callable::Callable;
use super::parser::Parser;
use super::interpreter::Interpreter;
use super::expr::Expr;
use super::resolver::Resolver;
use super::worng_class::WorngClass;
use super::worng_instance::WorngInstance;
use super::error::{ WorngError, ValueError, RuntimeError };

#[derive(Debug)]
pub struct Worng {
  pub had_error: bool,
  pub had_runtime_error: bool,
}

#[derive(Debug)]
pub enum WorngValue {
  Number(f64),
  String(String),
  Bool(bool),
  Func(Rc<dyn Callable>),
  Class(Rc<WorngClass>),
  Instance(Rc<RefCell<WorngInstance>>),
  Nil,
}

impl std::fmt::Display for WorngValue {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      WorngValue::Number(number) => write!(f, "{}", number),
      WorngValue::String(ref string) => write!(f, "{}", string),
      WorngValue::Bool(b) => write!(f, "{}", b),
      WorngValue::Func(_) => f.write_str("func"),
      WorngValue::Class(ref name) => write!(f,"{}", name),
      WorngValue::Instance(ref klass) => write!(f,"Instance: {}", klass.borrow()),
      WorngValue::Nil => f.write_str("nil"),
    }
  }
}

impl std::clone::Clone for WorngValue {
  fn clone(&self) -> WorngValue {
    match *self {
      WorngValue::Number(number) => WorngValue::Number(number),
      WorngValue::String(ref string) => WorngValue::String(string.clone()),
      WorngValue::Bool(b) => WorngValue::Bool(b),
      WorngValue::Nil => WorngValue::Nil,
      WorngValue::Func(ref func) => WorngValue::Func(func.clone()),
      WorngValue::Class(ref class) => WorngValue::Class(class.clone()),
      WorngValue::Instance(ref klass) => WorngValue::Instance(klass.clone()),
    }
  }
}


impl Worng {
  
  pub fn main(&mut self, args: Vec<&str> ) {
    
    if args.len() > 1 {

      writeln!(io::stdout(), "Usage: worng [script]");
      std::process::exit(64)
    } else if let Some(filename) = args.first() {
      if let Err(errors) = self.run_file(filename.to_string(), &mut io::stdout()) {
        println!("Error running file {}\n", filename);

        for err in errors {
          println!("{}", err);
        }
    }
    } else {
      self.run_prompt(&mut io::stdout()).unwrap();
    }
  }
  
  fn run_file(&mut self, path: String, writer: &mut io::Write) ->  Result<(), Vec<RuntimeError>> {

    let mut f = File::open(path).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    let writer = Rc::new(RefCell::new(writer));
    let mut interpreter = Interpreter::new(writer);
    run(&mut interpreter, contents)
  }
  
  fn run_prompt(&mut self, writer: &mut io::Write) -> Result<(), Box<Error>>{
    let writer = Rc::new(RefCell::new(writer));
    let mut interpreter = Interpreter::new(writer.clone());

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
      run(&mut interpreter, l);
    }
  
    rl.save_history("history.txt").unwrap();
  
    Ok(())
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

fn run(interpreter: &mut Interpreter, source: String) -> Result<(), Vec<RuntimeError>> {
  let mut scanner = Scanner::new(&source);
  let tokens: Vec<Token> = scanner.scan_tokens();
  
  let mut parser = Parser::new(tokens);

  let mut expression: Vec<Stmt> = parser.parse().unwrap();
  // let mut interpreter = Interpreter::new();

  // if self.had_error { return };

  let mut resolver = Resolver::new();
  resolver.resolve(&mut expression);

  match interpreter.interpret(expression) {
    Some(err) => Err(vec![err]),
    None => Ok(()),
  }
}

pub fn run_string(code: String) -> String {
  let output: Vec<u8> = Vec::new();
  let mut cursor = Cursor::new(output);

  let result = {
    let mut writer = Rc::new(RefCell::new(&mut cursor as &mut io::Write));
    let mut interpreter = Interpreter::new(writer.clone());
    run(&mut interpreter, code)
  };

  match result {
    Ok(_) => {
        let output = cursor.get_ref().clone();
        String::from_utf8(output).unwrap()
    }
    Err(errors) => errors
        .iter()
        .map(|error| error.to_string())
        .collect::<Vec<String>>()
        .join("\n"),
  }
}

impl WorngValue {
  pub fn subtract(&self, other: WorngValue) -> Result<WorngValue, ValueError> {
    match (self, other) {
      (&WorngValue::Number(left), WorngValue::Number(right)) => {
        Ok(WorngValue::Number(left - right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn add(&self, other: WorngValue) -> Result<WorngValue, ValueError>{
    match(self, other){
      (&WorngValue::Number(left), WorngValue::Number(right)) => {
        Ok(WorngValue::Number(left + right))
      },
      // NOTE: should be possible to concat string with number.
      (&WorngValue::String(ref left), WorngValue::String(ref right)) => {
        let mut s = String::from(left);
        s.push_str(right);
        Ok(WorngValue::String(s))
      },
      _ => Err(ValueError::TypeError) 
    }
  }

  pub fn divide(&self, other: WorngValue) -> Result<WorngValue, WorngError>{
    match(self, other) {
      (&WorngValue::Number(left), WorngValue::Number(0.0)) => {
        Err(WorngError::RuntimeError(RuntimeError::DivideInvalidType))
      },
      (&WorngValue::Number(left), WorngValue::Number(right)) => {
        Ok(WorngValue::Number(left / right))
      },
      _ => Err(WorngError::RuntimeError(RuntimeError::DivideInvalidType) )
    }
  }

  pub fn multiply(&self, other: WorngValue) -> Result<WorngValue, ValueError>{
    match(self, other){
      (&WorngValue::Number(left), WorngValue::Number(right)) => {
        Ok(WorngValue::Number(left * right))
      },
      (&WorngValue::Number(left), WorngValue::Bool(right)) => {
        Ok(WorngValue::Number(left * (right as i32 as f64) ))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn greater(&self, other: WorngValue) -> Result<WorngValue, ValueError> {
    match(self, other){
      (&WorngValue::Number(left), WorngValue::Number(right)) => {
        Ok(WorngValue::Bool(left > right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn greater_than(&self, other: WorngValue) -> Result<WorngValue, ValueError> {
    match(self, other){
      (&WorngValue::Number(left), WorngValue::Number(right)) => {
        Ok(WorngValue::Bool(left > right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn greater_equal(&self, other: WorngValue) -> Result<WorngValue, ValueError>{
    match(self, other){
      (&WorngValue::Number(ref left), WorngValue::Number(ref right) ) => {
        Ok(WorngValue::Bool( left >= right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn less_than(&self, other: WorngValue) -> Result<WorngValue, ValueError>{
    match(&self, &other){
      (WorngValue::Number(left ), WorngValue::Number(right)) => {
        Ok(WorngValue::Bool( left < right ))
      },
     _ => Err(ValueError::TypeError)
    }
  }

  pub fn less_equal(&self, other: WorngValue) -> Result<WorngValue, ValueError> {
    match(&self, &other) {
      (WorngValue::Number(left), WorngValue::Number(right)) => {
      Ok(WorngValue::Bool(left <= right))
      },
      _ => Err(ValueError::TypeError)
    }
  }

  pub fn bang_equal(&self, other: WorngValue) -> Result<WorngValue, ValueError> {
    Ok(WorngValue::Bool( !self.is_equal(&other)))
  }

  pub fn equal_equal(&self, other: WorngValue) -> Result<WorngValue, ValueError>{
    Ok(WorngValue::Bool( self.is_equal(&other)))
  }

  pub fn is_equal(&self, other: &WorngValue) -> bool {
    let boo = match (&self , &other) {
      (&WorngValue::Nil, &WorngValue::Nil) => { true},
      (&WorngValue::Bool(b), &WorngValue::Bool(other)) => b == other,
      (&WorngValue::String(ref string), &WorngValue::String(ref other)) => string == other,
      (&WorngValue::Number(num), &WorngValue::Number(other)) => num == other,
      (&WorngValue::Func(ref f), &WorngValue::Func(ref other)) => Rc::ptr_eq(f, other),
      _ => false,
    };

    boo
  }

  pub fn negate_number(&self) -> Result<WorngValue, ValueError> {
    if let WorngValue::Number(ref num) = *self {
      Ok(WorngValue::Number(-num))
    } else {
      Err(ValueError::TypeError)
    }
  }

  pub fn negate(&self) -> Result<WorngValue, ValueError> {
    Ok(WorngValue::Bool(!self.is_truthy())) 
  }

  pub fn is_truthy(&self) -> bool {
    match *self {
      WorngValue::Nil => false,
      WorngValue::Bool(b) => b,
      _ => true,
    }
  }

  pub fn get_callable(&self) -> Option<Rc<dyn Callable>> {
    match *self {
      WorngValue::Func(ref func) => Some(func.clone()),
      WorngValue::Class(ref klass) => Some(klass.clone()),
      _ => None,
    }
  }

}


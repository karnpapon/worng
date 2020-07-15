use std::rc::Rc;
use std::cell::RefCell;
use std::error::{Error};

use super::expr::Expr;
use super::token_type::TokenType;
use super::token::{Literal, Token};
use super::yuth::{YuthValue};
use super::statement::Stmt;
use super::error::{YuthError, ParsingError, RuntimeError };
use super::environment::Environment;

pub struct Interpreter{
  // value: Box<Expr>
  environment: Rc<RefCell<Environment>>,
}

impl Interpreter {

  pub fn new() -> Self{
    Interpreter{
      environment: Rc::new(RefCell::new(Environment::new()))
     }
  }

  pub fn check_number_operand(&self, _operator: &Token, operand: &YuthValue) -> Result<(), RuntimeError> {
    match operand {
      YuthValue::Number(value) => { return Ok(()) },
      _ => Err(RuntimeError::InternalError("Operand must be a number.".to_string()))
    }
  }

  pub fn interpret(&mut self, statement: Vec<Stmt>) -> Option<RuntimeError>{
    for stmt in statement.iter() {
      if let Err(err) = self.interpret_statement(stmt) {
        return Some(err);
      }
    }
    None
  }

  pub fn interpret_statement(&mut self, statement: &Stmt) -> Result<Option<YuthValue>, RuntimeError> {
    match *statement {
      Stmt::Print(ref expr) => { 
        return self.interpret_expression(expr)
        .map(|value| { 
          println!("print value = {:?}", &value);
          None
         }
        )
      },
      Stmt::Expr(ref expr) => self.interpret_expression(expr).map(|_| None),
      Stmt::Var(ref token, ref expr) => self.interpret_expression(expr).map(|value| {
        self.environment.borrow_mut().define(token.lexeme.clone(), value);
        None
      }),
      Stmt::Block(ref statements) => {
        let env = Environment::enclose(self.environment.clone());
        self.interpret_block(statements, RefCell::new(env))
      }
    }
  }

  pub fn interpret_block( &mut self, statements: &Vec<Stmt>, _environment: RefCell<Environment>) -> Result<Option<YuthValue>, RuntimeError> {
    let mut return_value = None;
    let previous = self.environment.clone();
    self.environment = Rc::new(_environment);

    for ref stmt in statements {
      return_value = self.interpret_statement(stmt)?;

      // if return_value.is_some() {
      //   break;
      // }
    }

    self.environment = previous;
    Ok(return_value)
  }

  pub fn interpret_expression(&mut self, expression: &Expr) -> Result<YuthValue, RuntimeError>  {
    match *expression {
      Expr::Literal(ref literal) => {
        if let Some(value) = literal.value() {
          Ok(value)
        } else {
          Err(RuntimeError::InternalError("Invalid literal - no value".to_string()))
        }
      },
      Expr::Binary(ref left, ref operator, ref right) => {
        let l = self.interpret_expression(left)?;
        let r = self.interpret_expression(right)?;

        match operator.token_type {
          TokenType::Minus => { 
            return l.subtract(r)
                    .map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())) 
            },
          TokenType::Slash => return l.divide(r).map_err(|_| RuntimeError::DivideByZero(operator.clone())),
          TokenType::Star => return l.multiply(r).map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
          TokenType::Plus => return l.add(r).map_err(|_| RuntimeError::AddNonNumbers(operator.clone())),
          TokenType::Greater => return l.greater_than(r).map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
          TokenType::GreaterEqual => return l.greater_equal(r).map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
          TokenType::Less => return l.less_than(r).map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
          TokenType::LessEqual => return l.less_equal(r).map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
          TokenType::BangEqual => return l.bang_equal(r).map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
          TokenType::EqualEqual => return l.equal_equal(r).map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
          _ => return Err(RuntimeError::InternalError("operator not support".to_string()))
        };
      },
      Expr::Unary( ref operator,ref right ) => {
        let r = self.interpret_expression(right)?;
        match operator.token_type {
          TokenType::Minus => { 
            match self.check_number_operand(operator, &r){
              Ok(()) => return r.negate_number().map_err(|_| RuntimeError::RuntimeError),
              Err(e) => Err(e)
            }
          },
          TokenType::Bang => return r.negate().map_err(|_| RuntimeError::RuntimeError),
          _ => return Err(RuntimeError::InternalError("invalid operator for unary.".to_string()))
        }
      },
      Expr::Var(ref token, ref value) => {
        match self.environment.borrow().get_value(&token) {
          Ok(value) => Ok(value.clone()),
          Err(_) => Err(RuntimeError::RuntimeError)
        }
      },
      Expr::Assign(ref token, ref expr, ref value) => {
        let val = self.interpret_expression(expr)?;
        match self.environment.borrow_mut().assign(&token.lexeme,val.clone()){
          Ok(()) => Ok(val),
          Err(e) => Err(RuntimeError::UndefinedVariable(token.clone())),
        }
      },
      Expr::Grouping(ref expr) => {
        self.interpret_expression(&expr)
      }
    }
  }

}
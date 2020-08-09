use std::rc::Rc;
use std::cell::RefCell;
use std::error::{Error};
use std::collections::HashMap;

use super::expr::Expr;
use super::token_type::TokenType;
use super::token::{Literal, Token};
use super::yuth::{YuthValue};
use super::statement::Stmt;
use super::error::{YuthError, ParsingError, RuntimeError };
use super::yuth_function::YuthFunction;
use super::yuth_class::YuthClass;
use super::environment::Environment;

pub struct Interpreter{
  pub globals: Rc<RefCell<Environment>>,
  environment: Rc<RefCell<Environment>>,
  locals: HashMap<Expr, usize>
}

impl Interpreter {

  pub fn new() -> Self{
    let globals = Rc::new(RefCell::new(Environment::global()));

    Interpreter{
      globals: globals.clone(),
      environment: globals.clone(),
      locals: HashMap::new()
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
        return self.interpret_expression(expr).map(|value| { 
          let msg = format!("{}\n", value);
          print!("{}" ,&msg);
          None
         }
        )
      },
      Stmt::Expr(ref expr) => { self.interpret_expression(expr).map(|_| None) },
      Stmt::Var(ref token, ref expr) => self.interpret_expression(expr).map(|value| {
        self.environment.borrow_mut().define(token.lexeme.clone(), value);
        None
      }),
      Stmt::If(ref condition, ref then_branch, ref else_branch) => {
        self.interpret_expression(condition).and_then(|condition_result| {
          if condition_result.is_truthy() {
            self.interpret_statement(then_branch)
          } else if let Some(ref else_branch) = **else_branch {
            self.interpret_statement(else_branch)
          } else {
            Ok(None)
          }
        })
    }
      Stmt::While(ref condition, ref body) => {
        while self.interpret_expression(condition)?.is_truthy() {
          self.interpret_statement(body)?;
        }

        Ok(None)
      },
      Stmt::Block(ref statements) => {
        let env = Environment::enclose(self.environment.clone());
        self.interpret_block(statements, RefCell::new(env))
      },
      Stmt::Func(ref name, ref params, ref body) => {
        let function = YuthValue::Func(Rc::new(YuthFunction::new(statement.clone(), self.environment.clone()) ) );
        self.environment.borrow_mut().define(name.clone().lexeme, function);
        return Ok(None);
      },
      Stmt::Return(_, ref expr) => { 
        Ok(Some(self.interpret_expression(expr)?)) 
      },
      Stmt::Class(ref name, ref methods ) => {
        self.environment.borrow_mut().define(name.lexeme.clone(), YuthValue::Nil);
        let mut _methods: HashMap<String, YuthFunction> = HashMap::new();

        for method_stmt in methods {
          match method_stmt {
            &Stmt::Func(ref name, _, _) => {
                let method = YuthFunction::new(
                  method_stmt.clone(),
                    self.environment.clone(),
                    // name.lexeme == "init",
                );
                _methods.insert(name.lexeme.clone(), method);
            }
            _ => {
              return Err(RuntimeError::InternalError(
                "Found a non Stmt::Func as a method of a class".to_string(),
              ))
            }
          };
        };

        let klass = YuthClass::new(name.lexeme.clone(), _methods);
        self.environment.borrow_mut().assign(&name.lexeme, YuthValue::Class(Rc::new(klass) )).expect("class assign error");
        return Ok(None);
      }
    }
  }

  pub fn interpret_block( &mut self, statements: &Vec<Stmt>, _environment: RefCell<Environment>) -> Result<Option<YuthValue>, RuntimeError> {
    let mut return_value = None;
    let previous = self.environment.clone();
    self.environment = Rc::new(_environment);

    for ref stmt in statements {
      return_value = self.interpret_statement(stmt)?;

      if return_value.is_some() {
        break;
      }
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
          TokenType::Minus => return l.subtract(r) .map_err(|_| RuntimeError::SubtractNonNumbers(operator.clone())),
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
      Expr::Var(ref token, ref distance) => {

        // resolve here 
        match distance {
          Some(d) => {
            // get_at here.
            match self.environment.borrow_mut().get_at(*d, &token) {
              Ok(value) => Ok(value.clone()),
              Err(_) => Err(RuntimeError::RuntimeError)
            }
          },
          None => {
            match self.globals.borrow().get_value(&token) {
              Ok(value) => Ok(value.clone()),
              Err(_) => Err(RuntimeError::RuntimeError)
            }
          }
        }
        
      },
      Expr::Assign(ref token, ref expr, ref distance) => {
        let value = self.interpret_expression(expr)?;

        match distance {
          &Some(d) => match self.environment.borrow_mut().assign_at(d, &token, value.clone()) {
            Ok(()) => Ok(value.clone()),
            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
          },
          &None => match self.globals.borrow_mut().assign(&token.lexeme, value.clone()){
            Ok(()) => Ok(value.clone()),
            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
          }
        }
      },
      Expr::Logical(ref left, ref token, ref right) => {
        let left = self.interpret_expression(left)?;

        if token.token_type == TokenType::Or {
          if left.is_truthy() { return Ok(left) };
        } else {
          if !left.is_truthy() { return Ok(left) };
        }

        return self.interpret_expression(right);  
      },
      Expr::Grouping(ref expr) => {
        self.interpret_expression(&expr)
      },
      Expr::Call(ref call, ref paren, ref arguments) => { 
        let function = self.interpret_expression(call)?
                          .get_callable()
                          .ok_or_else(|| RuntimeError::CallOnNonCallable(paren.clone()))?;

        let mut _arguments = Vec::new();
        for argument in arguments { 
          _arguments.push(self.interpret_expression(argument)?);
        }

        if arguments.len() != function.arity() {
          return Err(RuntimeError::ArityError(function.arity(), arguments.len())); 
        }

        return function.call(self, _arguments);
      },
      Expr::Get(ref obj, ref name) => {
        let object = self.interpret_expression(obj).unwrap();
        match object {
          YuthValue::Instance( ob ) => Ok(ob.borrow().get(name.clone()).unwrap() ),
          _ => { Err(RuntimeError::RuntimeError) } // TODO: msg = "Only instances have properties.""
        }
      },
      Expr::Set(ref object, ref name, ref value) => {
        let object = self.interpret_expression(object).unwrap();

        match object {
          YuthValue::Instance(ref klass) => {
            let _value = self.interpret_expression(value).unwrap();
            klass.borrow_mut().set(name.clone(), _value.clone()); 
            Ok(_value)
          },
          _ => { Err(RuntimeError::RuntimeError)} // TODO: msg = ""Only instances have fields. "
        }
      }
    }
  }
}
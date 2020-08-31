use std::rc::Rc;
use std::io;
use std::cell::RefCell;
use std::error::{Error};
use std::collections::HashMap;

use super::expr::Expr;
use super::token_type::TokenType;
use super::token::{Literal, Token};
use super::worng_value::{WorngValue};
use super::statement::Stmt;
use super::error::{WorngError, ParsingError, RuntimeError };
use super::worng_function::WorngFunction;
use super::worng_class::WorngClass;
use super::environment::Environment;

pub struct Interpreter<'a>{
  pub globals: Rc<RefCell<Environment>>,
  environment: Rc<RefCell<Environment>>,
  locals: HashMap<Expr, usize>,
  writer: Rc<RefCell<&'a mut io::Write>>,
}

impl<'a> Interpreter<'a> {

  pub fn new(writer: Rc<RefCell<&'a mut io::Write>>) -> Self{
    let globals = Rc::new(RefCell::new(Environment::global()));

    Interpreter{
      globals: globals.clone(),
      environment: globals.clone(),
      locals: HashMap::new(),
      writer: writer
    }
  }

  pub fn check_number_operand(&self, _operator: &Token, operand: &WorngValue) -> Result<(), RuntimeError> {
    match operand {
      WorngValue::Number(value) => { return Ok(()) },
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

  pub fn interpret_statement(&mut self, statement: &Stmt) -> Result<Option<WorngValue>, RuntimeError> {
    match *statement {
      Stmt::Print(ref expr) => self.interpret_expression(expr).map(|val| {
        self.writer
            .borrow_mut()
            .write_all(format!("{}\n", val).as_ref())
            .expect("Error writing to stdout/writer");
        None
    }),
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
        let function = WorngValue::Func(Rc::new(WorngFunction::new(statement.clone(), self.environment.clone(), false ) ) );
        self.environment.borrow_mut().define(name.clone().lexeme, function);
        return Ok(None);
      },
      Stmt::Return(_, ref expr) => { 
        Ok(Some(self.interpret_expression(expr)?)) 
      },
      Stmt::Class(ref token, ref superclass,  ref method_statements) => {
        let mut methods = HashMap::new();
        let mut parent_env = None;

        // let mut _superclass = None;
        // let mut resolved_superclass = None;

        let resolved_superclass = if let &Some(ref superclass) = superclass {
          let superclass = match self.interpret_expression(superclass)? {
              WorngValue::Class(ref class) => class.clone(),
              _ => return Err(RuntimeError::InvalidSuperclass(token.clone())),
          };

          parent_env = Some(self.environment.clone());
          let mut env = Environment::enclose(self.environment.clone());
          env.define("super".to_string(), WorngValue::Class(superclass.clone()));
          self.environment = Rc::new(RefCell::new(env));

          Some(superclass)
        } else {
            None
        };
        
        // if let Some(_super) = superclass {
        //   _superclass = self.interpret_expression(_super).ok();
        //   if let WorngValue::Class(ref klass) = _superclass.unwrap() {
        //     resolved_superclass = Some(klass.clone());

        //     parent_env = Some(self.environment.clone());
        //     let mut env = Environment::enclose(self.environment.clone());
        //     env.define("super".to_string(), WorngValue::Class(klass.clone()));
        //     self.environment = Rc::new(RefCell::new(env));

        //     Some(superclass)
        //   } else {
        //     return Err(RuntimeError::InvalidSuperclass(token.clone()))
        //   }
        // }
    
        for method_statement in method_statements {
            match method_statement {
                &Stmt::Func(ref name, _, _) => {
                    let method = WorngValue::Func(Rc::new(WorngFunction::new(
                        method_statement.clone(),
                        self.environment.clone(),
                        name.lexeme == "init"
                    )));
                    methods.insert(name.lexeme.clone(), method);
                }
                _ => {
                    return Err(RuntimeError::InternalError(
                        "Found a non Stmt::Func as a method of a class".to_string(),
                    ))
                }
            };
        }

        let class = WorngValue::Class(Rc::new(WorngClass::new(
            token.lexeme.clone(),
            methods,
            resolved_superclass
        )));

        if superclass.is_some() {
          self.environment = parent_env.expect("When interpreting a subclass, a parent environment should always be present");
        }

        self.environment.borrow_mut().define(token.lexeme.clone(), class);

        Ok(None)
      },
      // Stmt::Class(ref name, ref methods ) => {
      //   self.environment.borrow_mut().define(name.lexeme.clone(), WorngValue::Nil);
      //   let mut methods_statements = HashMap::new();

      //   for method_stmt in methods {
      //     match method_stmt {
      //       &Stmt::Func(ref name, _, _) => {
      //           let _method = WorngValue::Func(Rc::new(WorngFunction::new(
      //             method_stmt.clone(),
      //               self.environment.clone(),
      //               // name.lexeme == "init",
      //           )));
      //           methods_statements.insert(name.lexeme.clone(), _method);
      //       }
      //       _ => {
      //         return Err(RuntimeError::InternalError(
      //           "Found a non Stmt::Func as a method of a class".to_string(),
      //         ))
      //       }
      //     };
      //   };

        
      //   let klass = WorngValue::Class(Rc::new(WorngClass::new(
      //     name.lexeme.clone(),
      //     methods_statements,
      //   )));
        
      //   self.environment.borrow_mut().assign(&name.lexeme, klass ).expect("class assign error");
      //   return Ok(None);
      // }
    }
  }

  pub fn interpret_block( &mut self, statements: &Vec<Stmt>, _environment: RefCell<Environment>) -> Result<Option<WorngValue>, RuntimeError> {
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

  pub fn interpret_expression(&mut self, expression: &Expr) -> Result<WorngValue, RuntimeError>  {
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
              Ok(()) => return r.negate_number().map_err(|_| RuntimeError::RuntimeError(operator.clone())),
              Err(e) => Err(e)
            }
          },
          TokenType::Bang => return r.negate().map_err(|_| RuntimeError::RuntimeError(operator.clone())),
          _ => return Err(RuntimeError::InternalError("invalid operator for unary.".to_string()))
        }
      },
      Expr::Var(ref token, ref distance) => match distance {
        &Some(distance) => match self.environment.borrow().get_at(distance, &token.lexeme) {
            Ok(value) => Ok(value.clone()),
            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
        },
        &None => match self.globals.borrow().get_value(&token.lexeme) {
            Ok(value) => Ok(value.clone()),
            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
        },
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
      Expr::Get(ref target, ref token) => {
        let resolved_target = self.interpret_expression(target)?;

        match resolved_target {
            WorngValue::Instance(ref instance) => Ok(instance.borrow().get(&token)?.clone()),
            _ => Err(RuntimeError::InvalidGetTarget(token.clone())),
        }
      }
    Expr::Set(ref target, ref token, ref expr) => {
        let resolved_target = self.interpret_expression(target)?;

        let value = match resolved_target {
            WorngValue::Instance(instance) => {
                let resolved_value = self.interpret_expression(expr)?;
                instance
                    .borrow_mut()
                    .set(token.clone(), resolved_value.clone());
                resolved_value.clone()
            }
            _ => return Err(RuntimeError::InvalidGetTarget(token.clone())),
        };

        Ok(value)
      },
      Expr::Super(_, ref method, ref distance) => match distance {
        &Some(distance) => {
          let superclass = self.environment
              .borrow()
              .get_at(distance, &"super".to_string())
              .expect("Couldn't find `super` when interpreting");
          let instance = self.environment
              .borrow()
              .get_at(distance - 1, &"this".to_string())
              .expect("Couldn't find `this` when interpreting `super` call");

          let superclass = match superclass {
              WorngValue::Class(ref class) => class,
              _ => {
                return Err(RuntimeError::InternalError(
                    "Couldn't extract WorngClass from WorngValue::Class".to_string(),
                ))
              }
          };

          let instance = match instance {
              WorngValue::Instance(ref instance) => instance,
              _ => {
                return Err(RuntimeError::InternalError(
                  "Couldn't extract WorngInstance from WorngValue::Instance".to_string(),
                ))
              }
          };

          let resolved_method = superclass.find_method(&method.lexeme, instance.clone());

          match resolved_method {
            Some(method) => Ok(WorngValue::Func(Rc::new(method))),
            None => Err(RuntimeError::UndefinedProperty(method.clone())),
          }
        }
        &None => Err(RuntimeError::InternalError( "Couldn't find distance to super reference".to_string())),
      },
      Expr::This(ref token, ref distance) => { 
        match distance {
        &Some(distance) => match self.environment.borrow().get_at(distance, &token.lexeme) {
            Ok(value) => Ok(value.clone()),
            Err(_) => { Err(RuntimeError::UndefinedVariable(token.clone())) },
        },
        &None => match self.globals.borrow().get_value(&token.lexeme) {
            Ok(value) => Ok(value.clone()),
            Err(_) => Err(RuntimeError::UndefinedVariable(token.clone())),
        },
      }
      }
    }
  }
}
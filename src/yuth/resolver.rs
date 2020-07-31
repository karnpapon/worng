use std::collections::HashMap;

use super::statement::Stmt;
use super::interpreter::Interpreter;
use super::error::RuntimeError;
use super::expr::Expr;
use super::token::{ Token, Literal };

#[derive(Clone)]
enum FunctionType {
  FUNCTION
}

/// basically, Resolver is to figure out how many "distance" the variables in the "scope" are.
/// it tells the interpreter how many scopes there are 
/// between the current scope and the scope where the variable is defined.
/// it not perform any arithmatic nor looping/short-curcuit branching statement.
/// it's only job is to "run once".
pub struct Resolver {
  scopes: Vec<HashMap<String, bool>>,
  current_function: Option<FunctionType>
}

impl Resolver {
  pub fn new() -> Resolver {
    Resolver{
      scopes: Vec::new(),
      current_function: None
    }
  }

  pub fn resolve(&mut self, statement: &mut Vec<Stmt>) {
    for ref mut stmt in statement {
      self.resolve_statement(stmt) 
    }
  }

  fn resolve_statement(&mut self, stmt: &mut Stmt) {
    match *stmt{
      Stmt::Block(ref mut statements) => {
        self.begin_scope();
        self.resolve(statements);
        self.end_scope();
      },
      Stmt::Var(ref token, ref mut expr) => {
        self.declare(token);
        self.resolve_expression(expr);
        self.define(token);
      },
      Stmt::Func(ref name, ref params, ref mut body) => {
        self.declare(name);
        self.define(name);
        self.resolve_function(params, body, Some(FunctionType::FUNCTION) );
      },
      Stmt::Class(ref name, ref methods) => {
        self.declare(name);
        self.define(name);
      },
      Stmt::Expr(ref mut expr) => {
        self.resolve_expression(expr);
      },
      Stmt::If(ref mut condition, ref mut then_stmt, ref mut else_stmt) => {
        self.resolve_expression(condition);
        self.resolve_statement(then_stmt);
        if let Some(ref mut else_statement) = **else_stmt { 
          self.resolve_statement(else_statement); 
        }
      },
      Stmt::Print(ref mut expr) => {
        self.resolve_expression(expr);
      },
      Stmt::Return(_, ref mut expr) => {
      {
        let function_type = self.current_function
            .as_ref()
            .expect("UnexpectedTokenError: Cannot use `return` at the top level.");
      }

      self.resolve_expression(expr)
    },
      Stmt::While(ref mut condition, ref mut body) => {
        self.resolve_expression(condition);
        self.resolve_statement(body);
      }
    }

  }

  fn resolve_function(&mut self, params: &Vec<Token>, body: &mut Stmt, func_type: Option<FunctionType>) {

    let enclosing_function = self.current_function.clone();
    self.current_function = func_type;

    self.begin_scope();
    for param in params {
      self.declare(&param);
      self.define(&param);
    }

    match body {
      &mut Stmt::Block(ref mut stmts) => for stmt in stmts {
        self.resolve_statement(stmt);
      },
      _ => panic!("The body of a function be Stmt::Block"),
    }

    
    self.end_scope();
    self.current_function = enclosing_function;
  }

  fn resolve_expression(&mut self, expr: &mut Expr){
    match *expr {
      Expr::Var(ref token, ref mut distance) => {
        if let Some(scope) = self.scopes.last() {
          if let Some(is_var_available) = scope.get(&token.lexeme) {
            if !is_var_available {
              // println!("error variable not available in scope.")
            }
          }
        }
        *distance = self.resolve_local(token.lexeme.clone());
      },
      Expr::Assign(ref token, ref mut expr, ref mut distance) => {
        self.resolve_expression(expr);
        *distance = self.resolve_local(token.lexeme.clone());
      },
      Expr::Binary(ref mut left , _ , ref mut right) => {
        self.resolve_expression(left);
        self.resolve_expression(right);
      },
      Expr::Call(ref mut callee, _ , ref mut arguments) => {
        self.resolve_expression(callee);
        for argument in arguments {
          self.resolve_expression(argument);
        }
      },
      Expr::Grouping(ref mut expression) => {
        self.resolve_expression(expression);
      },
      Expr::Literal(ref expression) => {},
      Expr::Logical(ref mut left, _ , ref mut right) => {
        self.resolve_expression(left);
        self.resolve_expression(right);
      },
      Expr::Unary(_, ref mut right) => {
        self.resolve_expression(right);
      }
    }

  }

  fn resolve_local(&self, lexeme: String) -> Option<usize> {
    for (i, scope) in self.scopes.iter().rev().enumerate() {
      if scope.contains_key(&lexeme) {
        return Some(i);
      }
    }

    None
  }

  
  fn begin_scope(&mut self) {
    let mut body = HashMap::new();
    body.insert(String::new(), false);
    self.scopes.push(body);
  }

  fn end_scope(&mut self) {
    self.scopes.pop();
  }

  fn declare(&mut self, name: &Token) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.lexeme.clone(), false);
    }
  }

  fn define(&mut self, name: &Token) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.lexeme.clone(), false);
    }
  }
}
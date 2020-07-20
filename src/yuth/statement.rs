use std::rc::Rc;

use super::expr::Expr;
use super::token::{ Literal, Token };
use super::error::ParsingError;

#[derive(Debug, Clone)]
pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  Var(Token, Expr),
  Block(Vec<Stmt>),
  If(Expr, Box<Stmt>, Box<Option<Stmt>>),
  While(Expr, Box<Stmt>),
  Func(Token, Vec<Token>, Box<Stmt>),
  Return(Token, Box<Expr>)
}

impl std::fmt::Display for Stmt {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self{
      Stmt::Expr(ref expr) => {
        write!(f, "Stmt::Expr = {}", expr)
      },
      Stmt::Print(ref expr) => {
        write!(f, "Stmt::Print = {}", expr)
      },
      Stmt::Var(ref token, ref expr) => {
        write!(f, "Stmt::Var = [token: {:?}] [value: {}]",token, expr)
      },
      Stmt::Block(ref stmt) => {
        write!(f, "block = {:?}", &stmt)
      },
      Stmt::If(ref expr, ref then_stmt, ref else_stmt) => {
        write!(f, "if condition = {:?} then {:?} else {:?}", expr, then_stmt, else_stmt)
      },
      Stmt::While(ref expr, ref stmt) => {
        write!(f, "while condition = {:?} then {:?} ", expr, stmt)
      },
      Stmt::Func(ref name, ref params, ref body) => {
        write!(f, "function name: {:?}, params: {:?}, body {:?} ", name, params, body)
      },
      Stmt::Return(ref keyword, ref value) => {
        write!(f, "return: keyword = {:?}, value = {:?}", keyword, value)
      }
    }
  }
}
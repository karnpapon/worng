use std::rc::Rc;

use super::expr::Expr;
use super::token::{ Literal, Token };
use super::error::ParsingError;

#[derive(Debug)]
pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  Var(Token, Expr),
  Block(Vec<Stmt>),
  If(Expr, Box<Stmt>, Box<Option<Stmt>>),
  While(Expr, Box<Stmt>)
}

impl std::fmt::Display for Stmt {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self{
      Stmt::Expr(ref expr) => {
        write!(f, "statement expression = {}", expr)
      },
      Stmt::Print(ref expr) => {
        write!(f, "print expresstion = {}", expr)
      },
      Stmt::Var(ref token, ref expr) => {
        write!(f, "var declaration = [token: {:?}] [value: {}]",token, expr)
      },
      Stmt::Block(ref stmt) => {
        write!(f, "block = {:?}", &stmt)
      },
      Stmt::If(ref expr, ref then_stmt, ref else_stmt) => {
        write!(f, "if condition = {:?} then {:?} else {:?}", expr, then_stmt, else_stmt)
      },
      Stmt::While(ref expr, ref stmt) => {
        write!(f, "while condition = {:?} then {:?} ", expr, stmt)
      }
    }
  }
}
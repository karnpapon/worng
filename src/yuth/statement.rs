use super::expr::Expr;
use super::token::{ Literal, Token };

#[derive(Debug)]
pub enum Stmt {
  Expr(Expr),
  Print(Expr),
  Var(Token, Expr),
  Block(Vec<Stmt>),
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
      }
    }
  }
}
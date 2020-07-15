use super::token::{Literal, Token};

use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr{
  Binary(Box<Expr>,Token,Box<Expr>),
  Unary(Token,Box<Expr>),
  Literal(Literal),
  Grouping(Box<Expr>),
  Var(Token, Option<usize>),
  Assign(Token, Box<Expr>, Option<usize>),
}


impl fmt::Display for Expr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match *self{
      Expr::Binary(ref left, ref operator, ref right) => {
        write!(f, "({} {} {})", operator.lexeme, left, right)
      },

      Expr::Unary(ref operator, ref right) => {
        write!(f, "({} {})", operator.lexeme, right) 
      },

      Expr::Grouping(ref expression) => {
        write!(f, "(group {})", expression) 
      }, 

      Expr::Literal(ref expression) => {
        write!(f, "{}", expression) 
      },
      Expr::Var(ref token, ref value) => {
        write!(f, "var {}", token.lexeme )
      },
      Expr::Assign(ref token, ref expr, _) => { 
       write!(f, "(assign {} {})", token.lexeme, expr)
      }
    }
  }
}
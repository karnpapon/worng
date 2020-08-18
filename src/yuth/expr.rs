use super::token::{Literal, Token};

use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr{
  Unary(Token,Box<Expr>),
  Literal(Literal),
  Binary(Box<Expr>,Token,Box<Expr>),
  Call(Box<Expr>, Token, Vec<Expr>),
  Get(Box<Expr>, Token),
  Grouping(Box<Expr>),
  Var(Token, Option<usize>),
  Assign(Token, Box<Expr>, Option<usize>),
  Set(Box<Expr>, Token, Box<Expr>),
  Logical(Box<Expr>, Token, Box<Expr>),
  This(Token, Option<usize>)
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
      Expr::Get(ref object, ref name) => {
        write!(f, "get obj = {}, name = {}", object, name.lexeme) 
      }, 
      Expr::Literal(ref expression) => {
        write!(f, "{}", expression) 
      },
      Expr::Var(ref token, ref value) => {
        write!(f, "var {}", token.lexeme )
      },
      Expr::Assign(ref token, ref expr, _) => { 
       write!(f, "(assign {} {})", token.lexeme, expr)
      },
      Expr::Set(ref object, ref name, ref value) => { 
       write!(f, "set {} {} to value: {}", object, name.lexeme , value )
      },
      Expr::Logical(ref left, ref token, ref right) => { 
       write!(f, "Logical = ({} {} {})", left, token.lexeme, right)
      },
      Expr::Call(ref callee, ref paren, ref arguments) => { 
       write!(f, "Call = {}, {:?}, {:?}", callee, paren, arguments)
      },
      Expr::This(ref keyword, _) => {
        write!(f, "this = {}", keyword.lexeme)
      }
    }
  }
}
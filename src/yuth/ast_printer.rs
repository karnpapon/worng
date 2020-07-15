use super::expr::Expr;
use super::token::{Token,Literal};
use super::token_type::TokenType;

pub fn main() {
  let expression = Expr::Binary(
    Box::new( Expr::Unary( 
        Token::new(TokenType::Minus, String::from("-"), None, 1), 
        Box::new(Expr::Literal(Literal::Number(123 as f64) ))
      )
    ), 
    Token::new(TokenType::Star, String::from("*"), None, 1),
    Box::new(Expr::Grouping( Box::new(Expr::Literal(Literal::Number(45.67))) ))
  );

  println!("expr = {}", expression);
}
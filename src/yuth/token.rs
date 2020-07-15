use super::token_type::TokenType;
use super::yuth::{YuthValue};

#[derive(Debug, PartialEq,Clone)]
pub enum Literal {
  Identifier(String),
  String(String),
  Number(f64),
  Bool(bool),
  Nil,
}


impl std::fmt::Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      Literal::Identifier(ref iden) => write!(f, "{}", iden),
      Literal::Number(ref number) => write!(f, "{}", number),
      Literal::String(ref string) => write!(f, "{}", string),
      Literal::Bool(ref b) => write!(f, "{}", b),
      Literal::Nil => write!(f, "nil"),
    }
  }
}

#[derive(Debug, Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub lexeme: String,
  pub literal: Option<Literal>,
  pub line: i32 
}

// impl std::fmt::Display for Token {
//   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//     match *self {
//       token_type => write!(f, "token_type: {}", token_type),
//       lexeme => write!(f, "lexeme: {}", lexeme),
//       literal => write!(f, "literal: {}", literal),
//       line => write!(f, "line: {}", line)
//     }
//   }
// }

impl Token{

  pub fn new( token_type: TokenType, lexeme: String, literal: Option<Literal> , line: i32) -> Token {
    Token {
      token_type: token_type,
      lexeme: lexeme,
      literal: literal,
      line: line
    }
  }

  fn to_string(&self) -> String {
    return format!("{:?} {:?} {:?}", self.token_type , self.lexeme, self.literal);
  }
}


impl Literal {
  pub fn value(&self) -> Option<YuthValue> {
    let v = match *self {
      Literal::Identifier(ref iden) => Some(YuthValue::Identifier(iden.to_string())),
      Literal::Number(number) => Some(YuthValue::Number(number)),
      Literal::String(ref string) => Some(YuthValue::String(string.to_string())),
      Literal::Bool(ref boo) => Some(YuthValue::Bool(boo.clone())),
      Literal::Nil => Some(YuthValue::Nil),
    };

    v
  }
}

  

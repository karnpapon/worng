use std::collections::{HashMap};
use std::convert::TryInto;

use super::token_type::*;
use super::token::*;

pub struct Scanner {
  source: Vec<char>,
  tokens: Vec<Token>,
  start: usize ,
  current: usize,
  line: i32
}

impl Scanner{
  pub fn new(source: &str) -> Self {
    Scanner{
      source: source.chars().collect(),
      tokens: Vec::<Token>::new(),
      line: 1,
      current: 0,
      start: 0
    }
  }

  pub fn is_keyword(&self, keys: &str) -> Option<TokenType>{
    match keys {
      "and"  => Some(TokenType::And),
      "class"  => Some(TokenType::Class),
      "else"  => Some(TokenType::Else),
      "false"  => Some(TokenType::False),
      "for"  => Some(TokenType::For),
      "fn"  => Some(TokenType::Fn),
      "if"  => Some(TokenType::If),
      "nil"  => Some(TokenType::Nil),
      "or"  => Some(TokenType::Or),
      "print"  => Some(TokenType::Print),
      "return"  => Some(TokenType::Return),
      "super"  => Some(TokenType::Super),
      "this"  => Some(TokenType::This),
      "true"  => Some(TokenType::True),
      "var"  => Some(TokenType::Var),
      "while"  => Some(TokenType::While),
      other => None
    }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }

    self.tokens.push(Token::new(TokenType::EOF, String::from(""), None, self.line));
    self.tokens.clone()
  }

  fn scan_token(&mut self) {
    let c: char = self.advance();
    match c {
      '(' => self.add_token(TokenType::LeftParen, None),
      ')' =>  self.add_token(TokenType::RightParen, None),
      '{' =>  self.add_token(TokenType::LeftBrace, None),
      '}' =>  self.add_token(TokenType::RightBrace, None),
      ',' =>  self.add_token(TokenType::Comma, None),
      '.' =>  self.add_token(TokenType::Dot, None),
      '-' =>  self.add_token(TokenType::Minus, None),
      '+' =>  self.add_token(TokenType::Plus, None),
      ';' =>  self.add_token(TokenType::Semicolon, None),
      '*' =>  {
        if !self.get_match('/') {
          self.add_token(TokenType::Star, None)
        } 
      }, 
      '!' =>  match self.get_match('=') {
        true => self.add_token(TokenType::BangEqual, None),
        false => self.add_token(TokenType::Bang, None)
      },
      '=' =>  match self.get_match('=') {
        true => self.add_token(TokenType::EqualEqual, None),
        false => self.add_token(TokenType::Equal, None)
      },
      '<' => match self.get_match('=') {
        true => self.add_token(TokenType::LessEqual, None),
        false => self.add_token(TokenType::Less, None)
      },
      '>' =>  match self.get_match('=') {
        true => self.add_token(TokenType::GreaterEqual, None),
        false => self.add_token(TokenType::Greater, None)
      },
      '/' => {
        if self.get_match('/') {
          while self.peek() != '\n' && !self.is_at_end() { 
            self.advance();
          }
        } else if self.get_match('*'){
          while self.peek() != '*' && self.peek_next() != '/' && !self.is_at_end() { 
            self.advance();
          } 
        } else {
          self.add_token(TokenType::Slash, None)
        }       
      },

      '"' => self.string(),
      ' ' | '\r' | '\t' => {},
      '\n' => self.line += 1,

      any => {
        if self.is_digit(any) {
          self.number();
        } else if is_alpha(any) {
          self.identifier();
        } else {
          println!("Line: {} Unexpected character.", self.line);
        }
      }  
    } 
  }

  fn is_digit(&self, c: char) -> bool {
    return c >= '0' && c <= '9';
  } 

  fn number(&mut self) {
    while self.is_digit(self.peek()) { 
     self.advance(); 
    }

    if self.peek() == '.' && self.is_digit(self.peek_next()) {
      self.advance();
      while self.is_digit(self.peek()) {
        self.advance();
      }
    }

    let num = Literal::Number(
      self.source[self.start..self.current]
      .iter()
      .collect::<String>()
      .parse::<f64>()
      .unwrap()
    );

    self.add_token( TokenType::Number, Some(num) );
  }

  fn identifier(&mut self) {
    while is_alpha_numeric(self.peek()) {
      self.advance();
    }

    let text = self.source[self.start..self.current].iter().collect::<String>();

    let t = match self.is_keyword(&text){
      Some(match_token) => match_token,
      None => TokenType::Identifier
    };

    let token = t.clone();
    self.add_token(token, None);
  }

  fn string(&mut self) {
    while self.peek() != '"' && !self.is_at_end() {
      if self.peek() == '\n' { self.line += 1 };
      self.advance();
    }

    if self.is_at_end() {
      println!("Unterminated string. Line: {}", self.line);
      return;
    }
    self.advance();
    let value = self.source[self.start + 1..self.current - 1].iter().collect::<String>();
    self.add_token(TokenType::String, Some(Literal::String(value)));
  }

  fn get_match(&mut self, expected: char) -> bool {
    if self.is_at_end() { return false };
    if self.source[self.current] != expected {
      return false;
    }; 
    self.current += 1;
    return true;
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.iter().count().try_into().unwrap()
  }

  fn advance(&mut self) -> char {
    self.current += 1;
    self.source[self.current - 1]
  }

  fn peek(&self) -> char {
    if self.is_at_end() { return '\0'} ;
    return self.source[self.current];
  }

  fn peek_next(&self) -> char {
    if self.current + 1 >= self.source.len() { 
      return '\0';
    };

    self.source[self.current + 1]
  } 

  fn add_token(&mut self, kind: TokenType, literal: Option<Literal>) {
    let text: String = self.source[self.start..self.current].iter().collect();
    self.tokens.push(Token::new(kind, text, literal, self.line));
  }
}


//  utils.
fn is_alpha(c: char)  -> bool {
  return (c >= 'a' && c <= 'z') ||
         (c >= 'A' && c <= 'Z') ||
          c == '_';
}

fn is_alpha_numeric(c: char) -> bool {
  return is_alpha(c) || is_alpha(c);
}

use super::yuth::Yuth;
use super::token::{ Literal, Token};
use super::token_type::TokenType;
use super::expr::Expr;
use super::statement::Stmt;
use super::error::{ ParsingError};

use std::result;
use std::error::{Error};

// type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Parser {
  tokens: Vec<Token>,
  current: usize
}

impl Parser{
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser{
      tokens: tokens,
      current: 0
    }
  }

  pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParsingError>> {
    let mut statements: Vec<Stmt> = Vec::new();
    let mut errors: Vec<ParsingError> = Vec::new();

    while !self.is_at_end() {
      match self.declaration() {
        Ok(stmt) => statements.push(stmt),
        Err(err) => errors.push(err),
      }
    }

    if errors.len() == 0 {
      Ok(statements)
    } else {
      Err(errors)
    }

  }

  pub fn declaration(&mut self) -> Result<Stmt, ParsingError> {
    
    let statement;
    if self.is_match(vec![TokenType::Var]){
      statement = self.var_declaration();
    } else {
      statement = self.statement();
    };

    match statement {
      Ok(stmt) => Ok(stmt),
      Err(err) => {
        self.synchronize();
        Err(err)
      }
    }
  }

  fn var_declaration(&mut self) -> Result<Stmt,ParsingError> {
    let key = self.consume(TokenType::Identifier, "Expect variable key.")?;

    let mut initializer = Expr::Literal(Literal::Nil);
    if self.is_match(vec![TokenType::Equal]) {
      initializer = match self.expression() {
        Ok(expr) => expr,
        Err(err) => return Err(err),
      };
    };

    match self.consume( TokenType::Semicolon, "Expect ';' after expression.") {
      Ok(_) => Ok(Stmt::Var(key, initializer )),
      Err(err) => Err(err),
    }
  }
  
  fn statement(&mut self) -> Result<Stmt, ParsingError> {
    if self.is_match(vec![TokenType::Print]) {
      self.print_statement()
    } else if self.is_match(vec![TokenType::LeftBrace]){
      self.block_statement()
    } else {
      self.expression_statement()
    }
  }

  fn block_statement(&mut self) -> Result<Stmt, ParsingError> {
    let mut statements : Vec<Stmt> = Vec::new();

    while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
      statements.push(self.declaration()?);
    }

    self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
    return Ok(Stmt::Block(statements));
  }

  fn expression_statement(&mut self) -> Result<Stmt, ParsingError> {
    let expr = self.expression()?;

    match self.consume( TokenType::Semicolon, "Expect ';' after expression.") {
      Ok(_) => Ok(Stmt::Expr(expr)),
      Err(err) => Err(err),
    }
  }


  fn expression(&mut self) -> Result<Expr, ParsingError> {
   self.assignment()
  }

  fn assignment(&mut self) -> Result<Expr, ParsingError>{
    let expr = self.equality()?;

    if self.is_match(vec![TokenType::Equal]) {
      let equals =  self.previous().clone();
      let value = self.assignment().unwrap();

      match expr {
        Expr::Var( token, _) => {
          return Ok(Expr::Assign(token, Box::new(value), None ));
        },
        _ => return Err(ParsingError::InvalidAssignmentError(equals))
      }
    }
    return Ok(expr);
  }

  fn equality(&mut self) -> Result<Expr, ParsingError> {
    let mut expr = self.comparison().unwrap();

    while self.is_match(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
      let operator = self.previous().clone();
      let right = self.comparison()?;
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right) );
    }

    return Ok(expr);
  }

  fn comparison(&mut self) -> Result<Expr, ParsingError> {
    let mut expr = self.addition().unwrap();

    while self.is_match(vec![
      TokenType::Greater, 
      TokenType::GreaterEqual, 
      TokenType::Less, 
      TokenType::LessEqual
      ]) {
      let operator = self.previous().clone();
      let right = self.addition()?;
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    return Ok(expr);
  }

  fn addition(&mut self) -> Result<Expr, ParsingError>  {
    let mut expr = self.multiplication().unwrap();

    while self.is_match(vec![TokenType::Minus, TokenType::Plus]) {
      let operator = self.previous().clone();
      let right = self.multiplication().unwrap();
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    return Ok(expr);
  }

  fn multiplication(&mut self) -> Result<Expr, ParsingError>  {
    let mut expr = self.unary().unwrap();

    while self.is_match(vec![TokenType::Slash, TokenType::Star]) {
      let operator = self.previous().clone();
      let right = self.unary().unwrap();
      expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
    }

    return Ok(expr);
  }

  fn unary(&mut self) -> Result<Expr, ParsingError>  {
    if self.is_match(vec![TokenType::Bang, TokenType::Minus]) {
      let operator = self.previous().clone();
      let right = self.unary().unwrap();
      return Ok(Expr::Unary(operator, Box::new(right)) );
    }

    
    return self.primary();
  }
  
  fn primary(&mut self) -> Result<Expr, ParsingError> {
    if self.is_match(vec![TokenType::False]) { return Ok(Expr::Literal(Literal::Bool(false))); }
    if self.is_match(vec![TokenType::True]) { return Ok(Expr::Literal(Literal::Bool(true))); }
    if self.is_match(vec![TokenType::Nil]) { return Ok(Expr::Literal(Literal::Nil)); }
    
    if self.is_match(vec![TokenType::Number, TokenType::String]) {
      return Ok(Expr::Literal(self.previous().literal.clone().unwrap()));
    }

    if self.is_match(vec![TokenType::Identifier]){
      return Ok(Expr::Var(self.previous().clone(), None));
    }
    
    if self.is_match(vec![TokenType::LeftParen]) {
      let expr = self.expression().unwrap();
      match self.consume(TokenType::RightParen, "Expect ')' after expression."){
        Ok(t) => return Ok(Expr::Grouping(Box::new(expr))),
        Err(err) => return Err(err), 
      }
    } else {
      Err(ParsingError::ParsingError) 
    }
  }

  fn print_statement(&mut self) -> Result<Stmt, ParsingError> {
    let expr = self.expression()?;

    match self.consume( TokenType::Semicolon, "Expect ';' after expression.") {
      Ok(_) => Ok(Stmt::Print(expr)),
      Err(err) => Err(err),
    }
  }

  fn is_match(&mut self, types: Vec<TokenType>) -> bool {
    for token_type in types {
      if self.check(&token_type) {
        self.advance();
        return true;
      }
    }

    return false;
  }

  fn check(&self, token: &TokenType) -> bool {
    if self.is_at_end() { return false; }
    return self.peek().token_type == *token;
  }

  fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParsingError> {
    if self.check(&token_type) { 
      return Ok(self.advance().clone());
    } else {
      return Err(ParsingError::ParsingError);
    };
  }

  fn synchronize(&mut self) {
    self.advance();

    while !self.is_at_end() {
      if self.previous().token_type == TokenType::Semicolon {return;};

      match self.peek().token_type {
        TokenType::Class |
        TokenType::Fn |
        TokenType::Var |
        TokenType::For |
        TokenType::If |
        TokenType::While |
        TokenType::Print |
        TokenType::Return => return,
        _ => {}
      }

      self.advance();
    }
  }

  

  fn advance(&mut self) -> &Token {
    if !self.is_at_end() { self.current += 1;}
    self.previous()
  }

  fn is_at_end(&self) -> bool {
    return self.peek().token_type == TokenType::EOF;
  }

  fn peek(&self) -> &Token {
    return self.tokens.get(self.current).unwrap();
  }

  fn previous(&self) -> &Token {
    return self.tokens.get(self.current - 1).unwrap();
  }
}


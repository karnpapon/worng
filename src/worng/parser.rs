use std::rc::Rc;

use super::worng_value::Worng;
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
    } else if self.is_match(vec![TokenType::Func]) {
      statement = self.function_declaration("function"); 
    } else if self.is_match(vec![TokenType::Class]) {
      statement = self.class_declaration(); 
    }else  {
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

  fn function_declaration(&mut self, kind: &str) -> Result<Stmt, ParsingError>{
    let name = self.consume(TokenType::Identifier, format!("Expect {} name.", kind).as_ref() )?;

    self.consume(TokenType::LeftParen, format!("Expect '(' after {} name." , kind).as_ref() )?;
    let mut parameters = Vec::new();
    if !self.check(&TokenType::RightParen) {
      parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
      while self.is_match(vec![TokenType::Comma]){
        if parameters.len() >= 10 {
          // error(self.peek(), "Cannot have more than 255 parameters.");
          println!("Cannot have more than 10 parameters.");
        }
        parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
      }
    }
    self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
    self.consume(TokenType::LeftBrace, format!("Expect \"{{\" before {} body." , kind).as_ref() )?;
    let body = self.block_statement()?;
    return Ok(Stmt::Func(name, parameters, Box::new(body) ));
  }

  fn  class_declaration(&mut self) -> Result<Stmt, ParsingError> {
    let name = self.consume(TokenType::Identifier, "Expect class name.")?;

    let mut superclass = None;

    if self.is_match(vec![TokenType::Less]) {
      self.consume(TokenType::Identifier, "Expect superclass name.")?;
      superclass = Some(Expr::Var(self.previous().clone(), Some(0) ));
    }

    self.consume(TokenType::LeftBrace, "Expect '{' before class body.")?;

    let mut methods: Vec<Stmt> = Vec::new();

    while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
      methods.push(self.function_declaration("method")?);
    }

    self.consume(TokenType::RightBrace, "Expect '}' after class body.")?;

    return Ok(Stmt::Class(name, superclass , methods));
  }
  
  fn statement(&mut self) -> Result<Stmt, ParsingError> {
    if self.is_match(vec![TokenType::Print]) {
      self.print_statement()
    } else if self.is_match(vec![TokenType::LeftBrace]){
      self.block_statement()
    } else if self.is_match(vec![TokenType::If]){
      self.if_statement()
    } else if self.is_match(vec![TokenType::While]){
      self.while_statement()
    } else if self.is_match(vec![TokenType::For]){
      self.for_statement()
    } else if self.is_match(vec![TokenType::Return]) {
      self.return_statement()
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

  fn  if_statement(&mut self) -> Result<Stmt, ParsingError> {
    self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
    let condition = self.expression()?;
    self.consume(TokenType::RightParen, "Expect ')' after if condition.")?; 

    let then_branch = self.statement()?;
    let mut else_branch = None;
    if self.is_match(vec![TokenType::Else]) {
      if let Ok(stmt) = self.statement(){
        else_branch = Some(stmt);
      }
    }

    return Ok(Stmt::If(condition, Box::new(then_branch), Box::new(else_branch) ));
  }

  fn while_statement( &mut self) -> Result<Stmt, ParsingError>{
    self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
    let condition = self.expression()?;
    self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
    let body = self.statement()?;

    println!("condition = {}", &condition);
    return Ok(Stmt::While(condition, Box::new(body)));    
  }

  fn for_statement(&mut self) -> Result<Stmt, ParsingError> {
    self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

    let initializer;
    if self.is_match(vec![TokenType::Semicolon]) {
      initializer = Ok(Stmt::Expr(Expr::Literal(Literal::Nil)));
    } else if self.is_match(vec![TokenType::Var]) {
      initializer = self.var_declaration();
    } else {
      initializer = self.expression_statement();
    }

    let mut condition = Ok(Expr::Literal(Literal::Nil));
    if !self.check(&TokenType::Semicolon) {
      condition = self.expression();
    }
    self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

    let mut increment = Ok(Expr::Literal(Literal::Nil));
    if !self.check(&TokenType::RightParen) {
      increment = self.expression();
    }
    self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

    let mut body = self.statement().unwrap();

    if let Ok(value) = &increment {
      body = Stmt::Block(vec![body, Stmt::Expr(increment.unwrap())] );
    }

    if let Ok(Expr::Literal(Literal::Nil)) = condition {
      condition = Ok(Expr::Literal(Literal::Bool(true)));
    }
    body = Stmt::While(condition.unwrap(), Box::new(body));

    if let Ok(init_val) = &initializer {
      body = Stmt::Block(vec![initializer.unwrap(), body]);
    }

    return Ok(body);
  }

  fn print_statement(&mut self) -> Result<Stmt, ParsingError> {
    let expr = self.expression()?;

    match self.consume( TokenType::Semicolon, "Expect ';' after expression.") {
      Ok(_) => Ok(Stmt::Print(expr)),
      Err(err) => Err(err),
    }
  }

  fn return_statement(&mut self) -> Result<Stmt, ParsingError>{
    
    let keyword = self.previous().clone();
    let mut value = Expr::Literal(Literal::Nil);
    
    if !self.check(&TokenType::Semicolon){
      value = self.expression()?;
    }

    self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;

    return Ok(Stmt::Return(keyword, Box::new(value)) );
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
    let expr = self.or()?;

    if self.is_match(vec![TokenType::Equal]) {
      let equals =  self.previous().clone();
      let value = self.assignment()?;

      match expr {
        Expr::Var( token, _) => {
          return Ok(Expr::Assign(token, Box::new(value), None ));
        },
        Expr::Get( ref object, ref name) => {
          return Ok( Expr::Set(object.clone(), name.clone(), Box::new(value)))
        },
        _ => return Err(ParsingError::InvalidAssignmentError(equals))
      }
    }
    return Ok(expr);
  }

  fn  or(&mut self) -> Result<Expr, ParsingError> {
    let mut expr = self.and()?;

    while self.is_match(vec![TokenType::Or]) {
      // not sure why using this expression directly is not working.
      // eg. Expr::Logical(Box::new(expr), self.previous().clone(), Box::new(right) ); is not working.
      let operator = self.previous().clone(); 
      let right = self.and()?;
      expr = Expr::Logical(Box::new(expr), operator, Box::new(right) );
    }

    return Ok(expr);
  }

  fn and(&mut self) -> Result<Expr, ParsingError> {
    let mut expr = self.equality()?;

    while self.is_match(vec![ TokenType::And ]) {
      let operator = self.previous().clone();
      let right = self.equality()?;
      expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
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

    
    return self.call();
  }

  fn call(&mut self) -> Result<Expr, ParsingError> {
    
    let mut expr = self.primary();

    loop { 
      if self.is_match(vec![TokenType::LeftParen]) {
        expr = self.finish_call(expr?);
      } else if self.is_match(vec![TokenType::Dot]){  
        let name = self.consume(TokenType::Identifier, "Expect property name after '.'." );
        expr = Ok(Expr::Get(Box::new(expr?), name? ));
      } else {
        break;
      }
    }

    return expr;
  }

  fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParsingError>{
    let mut arguments: Vec<Expr> = Vec::new();
    if !self.check(&TokenType::RightParen) {
      arguments.push(self.expression()?);

      while self.is_match(vec![TokenType::Comma]){
        if arguments.len() >= 10 {
          return Err(ParsingError::TooManyArgumentsError); // no needs to throw en error, just report, is fine.
        }
        arguments.push(self.expression()?);
      }
    }

    let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

    return Ok(Expr::Call(Box::new(callee), paren, arguments));
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

    if self.is_match(vec![TokenType::This]){
      return Ok(Expr::This(self.previous().clone(), None));
    }

    if self.is_match(vec![TokenType::Super]){
      let keyword = self.previous().clone();
      self.consume(TokenType::Dot, "Expect '.' after 'super'.");
      let _method = self.consume(TokenType::Identifier, "Expect superclass method name.")?;
      return Ok(Expr::Super(keyword.clone(), _method.clone(), None));
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
        TokenType::Func |
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


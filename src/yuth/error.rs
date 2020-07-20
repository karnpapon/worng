use super::token::Token;

pub enum YuthError{
  ValueError(ValueError),
  ParsingError(ParsingError),
  RuntimeError(RuntimeError) 
}

// TODO: add more error handling
pub enum ValueError {
  TypeError,
}

// TODO: add more error handling
#[derive(Debug)]
pub enum ParsingError {
  ParsingError,
  UnexpectedTokenError(Token, String),
  UnexpectedEofError,
  InvalidAssignmentError(Token),
  TooManyArgumentsError,
  TooManyParametersError,
  InternalError(String)
}

// TODO: add more error handling
#[derive(Debug)]
pub enum RuntimeError{
  RuntimeError,
  SubtractNonNumbers(Token),
  AddNonNumbers(Token),
  DivideByZero(Token),
  InternalError(String),
  UndefinedVariable(Token),
  ArityError(usize, usize),
  CallOnNonCallable(Token)
}

#[derive(Debug)]
pub enum EnvironmentError {
  EnvironmentError,
  UndefinedVariable(String),
}


// ---------------------------------------------------------------------


impl std::fmt::Display for EnvironmentError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
        EnvironmentError::UndefinedVariable(ref name) => {
          write!(f, "Undefined variable {}", name)
        },
        EnvironmentError::EnvironmentError => {
          write!(f, "environment error")
        }
    }
  }
}

impl std::error::Error for EnvironmentError {
  fn description(&self) -> &str {
    match *self {
      EnvironmentError::UndefinedVariable(_) => "UndefinedVariable",
      EnvironmentError::EnvironmentError => "EnvironmentError",
    }
  }
}
impl std::fmt::Display for RuntimeError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self{
      RuntimeError::RuntimeError => {
        write!(f, "runtime error")
      },
      RuntimeError::SubtractNonNumbers(ref expression) => {
        write!(f, "[Line: {}] subtract non-number: {}", expression.line, expression.lexeme)
      },
      RuntimeError::AddNonNumbers(ref expression) => {
        write!(f, "[Line: {}] add non-number: {}", expression.line, expression.lexeme)
      },
      RuntimeError::DivideByZero(ref expression) => {
        write!(f, "[Line: {}]: cannot divide by zero", expression.line)
      },
      RuntimeError::InternalError(ref string) => {
        write!(f, "Internal Error: {} ", string)
      },
      RuntimeError::UndefinedVariable(ref token) => {
        write!(f,  "[line {}] Undefined variable -> {}", token.line, token.lexeme)
      },
      RuntimeError::ArityError(ref expected, ref size ) => {
        write!(f,  "Expected {} arguments but got {}.", expected, size )
      },
      RuntimeError::CallOnNonCallable(ref token ) => {
        write!(f,  "call on non-callable: {}.", token.lexeme )
      }
    }
  }
}

impl std::fmt::Display for ParsingError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self{
      ParsingError::UnexpectedTokenError(ref token, ref string) => {
        write!(f, "[Line: {}] Unexpected token error: {} {}",token.line, string,token.lexeme )
      },
      ParsingError::UnexpectedEofError => {
        write!(f, "unexpected error")
      },
      ParsingError::InvalidAssignmentError(ref token) => {
        write!(f, "[Line: {}] invalid assingment error", token.line)
      },
      ParsingError::TooManyArgumentsError => {
        write!(f, "Cannot have more than 255 arguments.")
      },
      ParsingError::TooManyParametersError => {
        write!(f, "too many params error")
      },
      ParsingError::InternalError(ref string) => {
        write!(f, "Error: {}", string)
      },
      ParsingError::ParsingError => {
        write!(f, "Error: Parsing error")
      },
    }
  }
}
use std::cell::RefCell;
use std::rc::Rc;

use super::statement::Stmt;
use super::callable::Callable;
use super::interpreter::Interpreter;
use super::environment::Environment;
use super::yuth::YuthValue;
use super::error::RuntimeError;

#[derive(Debug, Clone)]
pub struct YuthFunction{
  declaration: Stmt,
  closure: Rc<RefCell<Environment>>
}


impl YuthFunction{
  pub fn new( declaration: Stmt, closure: Rc<RefCell<Environment>>) -> YuthFunction{
    match declaration{
      Stmt::Func(_,_,_) => {
        YuthFunction{
          declaration: declaration,
          closure: closure
        }
      },
      _ => panic!("Cannot build a Yuth Function with a Stmt other than Stmt::Func")
    }
  }
}


impl Callable for YuthFunction{
  fn call(&self, interpreter: &mut Interpreter, args: Vec<YuthValue>) -> Result<YuthValue, RuntimeError>{

    // each function has it's own environment
    // eg. recursive function has to have it's "enclosed" environment, 
    // just to collect it's own params, at certain stage of calling function.
    let mut environment = Environment::enclose(self.closure.clone());

    let (params, body)  = match self.declaration {
      Stmt::Func(ref name, ref params, ref body) => (params, body),
      _ => panic!("Cannot build a function statement other than Stmt::Func")
    };

    let body = match **body {
      Stmt::Block(ref statements) => statements,
      _ => panic!("Cannot build a get body statement other than Stmt::Block"),
    };

    for (i, param) in params.iter().enumerate() {
      environment.define(
          param.lexeme.clone(),
          args.get(i)
              .expect("Mismatched argument and parameter sizes")
              .clone(),
      );
    }

    let result = match interpreter.interpret_block(body, RefCell::new(environment))? {
      Some(res) => Ok(res),
      None => Ok(YuthValue::Nil)
    };

    return result;
  }

  fn arity(&self) -> usize{
    let params =  match self.declaration {
      Stmt::Func(_, ref params, _) => params.len(),
      _ => 0
    };
    
    return params;
  } 

  // TODO: "<fn " + declaration.name.lexeme + ">";
  fn func_to_string(&self) -> String{
    String::from("<function>")
  }
}
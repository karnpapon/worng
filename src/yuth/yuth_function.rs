use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

use super::statement::Stmt;
use super::callable::Callable;
use super::interpreter::Interpreter;
use super::environment::Environment;
use super::yuth::YuthValue;
use super::yuth_instance::YuthInstance;
use super::error::RuntimeError;

#[derive(Debug, Clone)]
pub struct YuthFunction{
  declaration: Stmt,
  closure: Rc<RefCell<Environment>>,
  is_initializer: bool
}


impl YuthFunction{
  pub fn new( declaration: Stmt, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> YuthFunction{
    match declaration{
      Stmt::Func(_,_,_) => {
        YuthFunction{
          declaration: declaration,
          closure: closure,
          is_initializer: is_initializer
        }
      },
      _ => panic!("Cannot build a Yuth Function with a Stmt other than Stmt::Func")
    }
  }

  pub fn bind(&self, instance: Rc<RefCell<YuthInstance>>) -> YuthFunction {
    let mut env = Environment::enclose(self.closure.clone());
    env.define("this".to_string(), YuthValue::Instance(instance.clone()));
    YuthFunction {
      declaration: self.declaration.clone(),
      closure: Rc::new(RefCell::new(env)),
      is_initializer: self.is_initializer
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


    if self.is_initializer {
      return Ok(self.closure
          .borrow()
          .get_at(0, &"this".to_string())
          .expect("Couldn't find reference to `this` in initializer"));
    }


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

  fn as_any(&self) -> &dyn Any{
    self
  }
}


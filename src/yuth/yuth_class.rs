use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::any::Any;

use super::callable::Callable;
use super::interpreter::Interpreter;
use super::error::{ RuntimeError, YuthError, ValueError };
use super::yuth::YuthValue;
use super::token::Token;
use super::yuth_function::YuthFunction;
use super::yuth_instance::YuthInstance;

#[derive(Debug, Clone)]
pub struct YuthClass {
  name: String,
  methods: HashMap<String, YuthValue>,
  superclass: Option<Rc<YuthClass>>
}

impl YuthClass {
  pub fn new(name: String, methods: HashMap<String, YuthValue>, superclass: Option<Rc<YuthClass>>,) -> YuthClass {
    YuthClass {
      name: name,
      methods: methods,
      superclass: superclass 
    }
  }

  pub fn find_method(&self, name: &str, instance: Rc<RefCell<YuthInstance>>) -> Option<YuthFunction> {
    self.methods
        .get(name)
        .map(|method| method.clone())
        .map(|method| match method {
            YuthValue::Func(ref callable) => callable
                .as_any()
                .downcast_ref::<YuthFunction>()
                .expect("Couldn't cast Callable to YuthFunc in YuthValue::Function")
                .bind(instance.clone()),
            _ => panic!("Can't get non-func as method from an instance"),
        })
        .or_else(|| {
          if let Some(superclass) = self.superclass.clone() {
            superclass.find_method(name, instance)
          } else {
            None
          }
      })
  }
}

impl std::fmt::Display for YuthClass {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Callable for YuthClass {

  fn call(
    &self, 
    interpreter: &mut Interpreter, 
    args: Vec<YuthValue>) 
    -> Result<YuthValue, RuntimeError>{

    let instance = YuthInstance::new(self.clone());
    match self.find_method("init", Rc::new(RefCell::new(instance.clone()))) {
      Some( _initializer ) => return _initializer.call(interpreter, args),
      None => return Ok(YuthValue::Instance(Rc::new( RefCell::new( instance )) ))
    };
  }

  fn arity(&self)  -> usize{
    let instance = YuthInstance::new(self.clone());
    match self.find_method("init", Rc::new(RefCell::new(instance) )){
      Some(func) => return func.arity(),
      None => return 0
    };
  }

  fn func_to_string(&self) -> String {
    return String::from("class func_to_string")
  }

  fn as_any(&self) -> &dyn Any{
    self
  }
}
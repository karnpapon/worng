use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::any::Any;

use super::callable::Callable;
use super::interpreter::Interpreter;
use super::error::{ RuntimeError, WorngError, ValueError };
use super::worng_value::WorngValue;
use super::token::Token;
use super::worng_function::WorngFunction;
use super::worng_instance::WorngInstance;

#[derive(Debug, Clone)]
pub struct WorngClass {
  name: String,
  methods: HashMap<String, WorngValue>,
  superclass: Option<Rc<WorngClass>>
}

impl WorngClass {
  pub fn new(name: String, methods: HashMap<String, WorngValue>, superclass: Option<Rc<WorngClass>>,) -> WorngClass {
    WorngClass {
      name: name,
      methods: methods,
      superclass: superclass 
    }
  }

  pub fn find_method(&self, name: &str, instance: Rc<RefCell<WorngInstance>>) -> Option<WorngFunction> {
    self.methods
        .get(name)
        .map(|method| method.clone())
        .map(|method| match method {
            WorngValue::Func(ref callable) => callable
                .as_any()
                .downcast_ref::<WorngFunction>()
                .expect("Couldn't cast Callable to WorngFunc in WorngValue::Function")
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

impl std::fmt::Display for WorngClass {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl Callable for WorngClass {

  fn call(
    &self, 
    interpreter: &mut Interpreter, 
    args: Vec<WorngValue>) 
    -> Result<WorngValue, RuntimeError>{

    let instance = WorngInstance::new(self.clone());
    match self.find_method("init", Rc::new(RefCell::new(instance.clone()))) {
      Some( _initializer ) => return _initializer.call(interpreter, args),
      None => return Ok(WorngValue::Instance(Rc::new( RefCell::new( instance )) ))
    };
  }

  fn arity(&self)  -> usize{
    let instance = WorngInstance::new(self.clone());
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
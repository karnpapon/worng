use super::callable::Callable;
use super::interpreter::Interpreter;
use super::error::RuntimeError;
use super::yuth::YuthValue;
use super::yuth_instance::YuthInstance;

#[derive(Debug, Clone)]
pub struct YuthClass {
  name: String
}

impl YuthClass {
  pub fn new(name: String) -> YuthClass {
    YuthClass {
      name: name
    }
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
      return Ok(YuthValue::Instance(instance));
  }

  fn arity(&self)  -> usize{
    return 0
  }
  fn func_to_string(&self) -> String {
    return String::from("class func_to_string")
  }
}
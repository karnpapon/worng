use super::yuth_class::YuthClass;

#[derive(Debug, Clone)]
pub struct YuthInstance {
  klass: YuthClass
}

impl YuthInstance {
  pub fn new(klass: YuthClass) -> YuthInstance {
    YuthInstance {
      klass: klass
    }
  }
}

impl std::fmt::Display for YuthInstance {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.klass)
  }
}
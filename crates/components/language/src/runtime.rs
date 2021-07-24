use std::collections::HashMap;

use crate::code::node::Code;

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RuntimeError {}

#[derive(Clone, PartialEq, Debug)]
pub struct Computed(pub Code);

pub trait Runtime {
    fn run(&self, code: &Code) -> Result<Code, RuntimeError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuntimeId(pub String);

impl<T: Into<String>> From<T> for RuntimeId {
    fn from(from: T) -> Self {
        Self(from.into())
    }
}

#[derive(Default)]
pub struct Runtimes {
    runtimes: HashMap<RuntimeId, Box<dyn Runtime + Send + Sync>>,
    default_id: Option<RuntimeId>,
}

impl Runtimes {
    pub fn new() -> Self {
        Self {
            runtimes: HashMap::new(),
            default_id: None,
        }
    }

    pub fn insert<T: Into<RuntimeId>, B: Runtime + Send + Sync + 'static>(
        &mut self,
        id: T,
        runtime: B,
    ) {
        self.runtimes.insert(id.into(), Box::new(runtime));
    }

    pub fn get_mut(&mut self, id: &RuntimeId) -> Option<&mut Box<dyn Runtime + Send + Sync>> {
        self.runtimes.get_mut(id)
    }

    pub fn get_default(&mut self) -> Option<&mut Box<dyn Runtime + Send + Sync>> {
        if let Some(default_id) = &self.default_id {
            self.runtimes.get_mut(default_id)
        } else {
            None
        }
    }

    pub fn set_default(&mut self, id: &RuntimeId) {
        self.default_id = Some(id.clone());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    pub struct TestRuntime;

    impl Runtime for TestRuntime {
        fn run(&self, _: &Code) -> Result<Code, RuntimeError> {
            todo!()
        }
    }

    #[test]
    fn set_runtime() {
        let mut runtimes = Runtimes::new();
        runtimes.insert("primary", TestRuntime);
        assert!(runtimes.runtimes.get(&"primary".into()).is_some());
    }

    #[test]
    fn get_runtime() {
        let mut runtimes: HashMap<RuntimeId, Box<dyn Runtime + Send + Sync + 'static>> =
            HashMap::new();
        runtimes.insert("primary".into(), Box::new(TestRuntime));
        let mut runtimes = Runtimes {
            runtimes,
            ..Default::default()
        };
        assert!(runtimes.get_mut(&"primary".into()).is_some());
    }

    #[test]
    fn default_runtime_none() {
        let mut runtimes = Runtimes::new();
        runtimes.insert("primary", TestRuntime);
        assert!(runtimes.get_default().is_none());
    }

    #[test]
    fn default_runtime_some() {
        let mut runtimes = Runtimes::new();
        runtimes.insert("primary", TestRuntime);
        runtimes.set_default(&"primary".into());
        assert!(runtimes.get_default().is_some());
    }
}

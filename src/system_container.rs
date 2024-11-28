use crate::systems::System;

#[derive(Default)]
pub struct SystemContainer {
    systems: Vec<Box<dyn System>>,
}

impl SystemContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<T: System + 'static>(&mut self, system: T) {
        self.systems.push(Box::new(system));
    }

    pub fn all(&self) -> &[Box<dyn System>] {
        &self.systems
    }
}

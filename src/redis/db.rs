use std::io::Error;

pub trait MemoryDatabase: Sync + Send {
    fn set(&mut self, key: &str, value: &str) -> Result<(), Error>;
    fn get(&self, key: &str) -> Option<String>;
    fn del(&mut self, key: &str) -> Option<String>;
}

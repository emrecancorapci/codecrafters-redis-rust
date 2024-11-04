use std::{collections::HashMap, io::Error};

use crate::redis::db::MemoryDatabase;

pub struct MemDB {
    data: HashMap<String, String>,
}

impl MemDB {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl MemoryDatabase for MemDB {
    fn set(&mut self, key: &str, value: &str) -> Result<(), Error> {
        self.data.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).map(|v| v.to_string())
    }

    fn del(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}
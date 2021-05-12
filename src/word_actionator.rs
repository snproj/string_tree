use std::sync::Arc;
use std::sync::Mutex;

use crate::vson::VSON;
use crate::trav_actionator::*;

#[derive (Debug)]
pub struct WordActionResult {
    content: String,
}

impl WordActionResult {
    fn new() -> WordActionResult {
        WordActionResult {
            content: "This is a WordActionResult string".to_string(),
        }
    }
}

#[derive (Debug)]
pub struct WordActionator {
}

impl WordActionator {
    pub fn new() -> WordActionator {
        WordActionator {
        }
    }
}

impl TravActionator for WordActionator {
    type TravActionResult = WordActionResult;
    fn commence_action(&self, target: Arc<Mutex<VSON>>) -> Option<Arc<Mutex<WordActionResult>>>{
        let new = WordActionResult::new();
        Some(Arc::new(Mutex::new(new)))
    }
}
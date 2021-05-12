use std::sync::Arc;
use std::sync::Mutex;

#[derive (Debug)]
pub enum VSON {
    VSO(String, Option<Arc<Mutex<VSON>>>, Option<Arc<Mutex<VSON>>>),
    Noun(String),
}
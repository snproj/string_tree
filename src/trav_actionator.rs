use std::sync::Arc;
use std::sync::Mutex;
use crate::vson::*;

pub trait TravActionator {
    type TravActionResult;
    fn decide_action(&self, target: Arc<Mutex<VSON>>) -> Option<Arc<Mutex<Self::TravActionResult>>>;
}
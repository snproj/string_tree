use std::sync::Arc;
use std::sync::Mutex;
use crate::vson::VSON;
use crate::trav_actionator::*;

#[derive (Hash, PartialEq, Eq, Debug, Clone)]
pub enum TravBehavior {
    Verbal,
    Nominal,
    EndOfFile,
    PastEnd,
}

pub trait TravDict {
    type TravType;
    type Action: TravActionator;
    fn get_type(&self, word: &String) -> Self::TravType;
    fn get_trav_behavior(&self, word: &String) -> TravBehavior;

    fn invoke_actionator(&self, target: Arc<Mutex<VSON>>) -> Option<Arc<Mutex<<Self::Action as TravActionator>::TravActionResult>>>;
}
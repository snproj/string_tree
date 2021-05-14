use std::sync::Arc;
use std::sync::Mutex;
use crate::vson::*;
use crate::trav_dict::*;

pub trait TravActionator<T: TravDict> {
    type TravActionResult;
    fn commence_action(&self, target: Arc<Mutex<VSON>>, dict: Arc<Mutex<T>>) -> Option<Arc<Mutex<Self::TravActionResult>>>;
}
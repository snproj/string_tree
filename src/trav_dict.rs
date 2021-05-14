#[derive (Hash, PartialEq, Eq, Debug, Clone)]
pub enum TravBehavior {
    Verbal,
    Nominal,
    EndOfFile,
    PastEnd,
}

pub trait TravDict {
    type TravType;
    fn get_type(&self, word: &String) -> Self::TravType;
    fn get_trav_behavior(&self, word: &String) -> TravBehavior;
}
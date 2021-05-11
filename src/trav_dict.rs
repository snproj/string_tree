#[derive (Hash, PartialEq, Eq, Debug, Clone)]
pub enum TravBehavior {
    Verbal,
    Nominal,
    END_OF_FILE,
    PAST_END,
}

pub trait TravDict{
    type TravType;
    fn get_type(&self, word: &String) -> Self::TravType;
    fn get_trav_behavior(&self, word: &String) -> TravBehavior;
}
use crate::trav_dict::*;
use std::collections::HashMap;

#[derive (Debug)]
pub struct WordDict {
    type_dict: HashMap<String, WordType>,
    behavior_dict: HashMap<WordType, TravBehavior>,
}

impl WordDict {
    pub fn new() -> WordDict{
        let standard_type_dict: HashMap<String, WordType> = [
            ("eat".to_string(), WordType::Action),
            ("like".to_string(), WordType::Action),
            ("birth".to_string(), WordType::Action),

            ("and".to_string(), WordType::Conj),
            ("adv.".to_string(), WordType::Conj),
            ("in".to_string(), WordType::Conj),
            ("adj.".to_string(), WordType::Conj),

            ("free".to_string(), WordType::Adj),
            ("equal".to_string(), WordType::Adj),
            ("human".to_string(), WordType::Adj),
            ("all".to_string(), WordType::Adj),

            ("I".to_string(), WordType::Noun),
            ("apple".to_string(), WordType::Noun),
            ("pear".to_string(), WordType::Noun),
            ("John".to_string(), WordType::Noun),
            ("Mike".to_string(), WordType::Noun),
            ("dignity".to_string(), WordType::Noun),
            ("right".to_string(), WordType::Noun),
            ("pass.".to_string(), WordType::Noun),
            ("being".to_string(), WordType::Noun),

            ("END_OF_FILE".to_string(), WordType::END_OF_FILE),
            ("PAST_END".to_string(), WordType::PAST_END),
        ].iter()
         .cloned()
         .collect();

        let standard_behavior_dict: HashMap<WordType, TravBehavior> = [
            (WordType::Action, TravBehavior::Verbal),
            (WordType::Conj, TravBehavior::Verbal),

            (WordType::Adj, TravBehavior::Nominal),
            (WordType::Noun, TravBehavior::Nominal),

            (WordType::END_OF_FILE, TravBehavior::END_OF_FILE),
            (WordType::PAST_END, TravBehavior::PAST_END),
        ].iter()
         .cloned()
         .collect();
        
        WordDict {
            type_dict: standard_type_dict,
            behavior_dict: standard_behavior_dict,
        }
    }
}


impl TravDict for WordDict {
    type TravType = WordType;

    fn get_type(&self, word: &String) -> WordType {
        match self.type_dict.get(word) {
            Some(thing) => thing.clone(),
            None => panic!("SUNRISE_ERROR: WordType not assigned to: {}", &word)
        }
    }

    fn get_trav_behavior(&self, word: &String) -> TravBehavior {
        match self.behavior_dict.get(&self.get_type(word)) {
            Some(thing) => thing.clone(),
            None => panic!("SUNRISE_ERROR: WordBehavior not assigned to: {}", &word)
        }
    }
}

#[derive (Hash, PartialEq, Eq, Debug, Clone)]
pub enum WordType {
    Action,
    Adj,
    Adv,
    Conj,
    Noun,
    END_OF_FILE,
    PAST_END,
}

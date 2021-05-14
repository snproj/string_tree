use std::sync::Arc;
use std::sync::Mutex;

use crate::vson::VSON;
use crate::trav_actionator::*;
use crate::trav_dict::*;

#[derive (Debug)]
pub struct AdvLog {
    adverb: String,
}

#[derive (Debug)]
pub struct AdjLog {
    adjective: String,
}

#[derive (Debug)]
pub struct VerbLog {
    is_indiv: bool,
    links: Vec<Arc<Mutex<VerbLog>>>,

    verb: Option<String>,
    adverbs: Vec<Arc<Mutex<AdvLog>>>,
}

#[derive (Debug)]
pub struct PlaceLog {
    place: String,
    adjectives: Vec<Arc<Mutex<AdjLog>>>,
    events: Vec<Arc<Mutex<EventLog>>>,
}

#[derive (Debug)]
pub struct EventLog {
    subj_actor: ActorLog,
    verb: VerbLog,
    obj_actor: ActorLog,

    time: TimeLog,
    place: PlaceLog,

    desc: Option<String>,
}

#[derive (Debug)]
pub struct TimeLog {
    time: i64,
    events: Vec<Arc<Mutex<EventLog>>>,

    desc: Option<String>,
}

#[derive (Debug)]
pub struct ActorLog {
    is_indiv: bool,
    links: Vec<Arc<Mutex<ActorLog>>>,

    actor: Option<String>,
    adjectives: Vec<Arc<Mutex<AdjLog>>>,
}

#[derive (Debug)]
pub struct WordActionResult {
    events: Vec<Arc<Mutex<EventLog>>>,
    actors: Vec<Arc<Mutex<ActorLog>>>,
    verbs: Vec<Arc<Mutex<VerbLog>>>,
    times: Vec<Arc<Mutex<TimeLog>>>,
    places: Vec<Arc<Mutex<PlaceLog>>>,

    adjs: Vec<Arc<Mutex<AdjLog>>>,
    advs: Vec<Arc<Mutex<AdvLog>>>,
}

impl WordActionResult {
    fn new() -> WordActionResult {
        WordActionResult {
            events: Vec::new(),
            actors: Vec::new(),
            verbs: Vec::new(),
            times: Vec::new(),
            places: Vec::new(),
            adjs: Vec::new(),
            advs: Vec::new(),
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

    pub fn pprint(thing: &VSON, tab_count: usize) {
        //println!("PPRINT START");
        let tabs = "\t".repeat(tab_count);
        let tabs_plus_one = tabs.clone() + "\t";
        match thing {
            VSON::VSO(value, Some(subj_vson), Some(obj_vson)) => {
                println!("{tabs}{value}", value=value, tabs=tabs);

                let borrow_subj = &mut *subj_vson.lock().unwrap();
                match borrow_subj {
                    VSON::VSO(_,_,_) => {
                        Self::pprint(borrow_subj, tab_count + 1);
                    },
                    VSON::Noun(value) => {
                        println!("{tabs_plus_one}{value}", value=value, tabs_plus_one=tabs_plus_one);
                    }
                }

                let borrow_obj = &mut *obj_vson.lock().unwrap();
                match borrow_obj {
                    VSON::VSO(_,_,_) => {
                        Self::pprint(borrow_obj, tab_count);
                    },
                    VSON::Noun(value) => {
                        println!("{tabs}{value}", value=value, tabs=tabs);
                    }
                }
            },

            VSON::VSO(value, Some(subj_vson), None) => {
                println!("{tabs}{value}", value=value, tabs=tabs);

                let borrow_subj = &mut *subj_vson.lock().unwrap();
                match borrow_subj {
                    VSON::VSO(_,_,_) => {
                        Self::pprint(borrow_subj, tab_count + 1);
                    },
                    VSON::Noun(value) => {
                        println!("{tabs_plus_one}{value}", value=value, tabs_plus_one=tabs_plus_one);
                    }
                }

                println!("{tabs}NONE\n", tabs=tabs);
            },

            VSON::VSO(value, None, None) => {
                println!("{tabs}{value}", value=value, tabs=tabs);

                println!("{tabs_plus_one}NONE", tabs_plus_one=tabs_plus_one);

                println!("{tabs}NONE", tabs=tabs);
            },

            VSON::VSO(_, None, _) => {
                panic!();
            },

            VSON::Noun(_) => {
                panic!();
            }
        }
        //println!("PPRINT END");
    }
}

impl<T: TravDict> TravActionator<T> for WordActionator {
    type TravActionResult = WordActionResult;
    fn commence_action(&self, target: Arc<Mutex<VSON>>, dict: Arc<Mutex<T>>) -> Option<Arc<Mutex<WordActionResult>>>{

        Self::pprint(&*target.lock().unwrap(), 0);

        
        let new = WordActionResult::new();
        Some(Arc::new(Mutex::new(new)))
    }
}
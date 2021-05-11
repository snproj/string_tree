use crate::trav_dict::*;
use std::sync::Arc;
use std::sync::Mutex;

#[derive (Debug)]
pub struct Traverser<T: TravDict> {
    string_vec: Vec<String>,
    vec_ptr: usize,
    store: Arc<Mutex<VSON>>,
    store_ptr: Arc<Mutex<VSON>>,
    prev_loc_stack: Vec<Arc<Mutex<VSON>>>,
    dict: Arc<Mutex<T>>,
}

impl<T: TravDict> Traverser<T> {
    pub fn new(orig: String, dict: Arc<Mutex<T>>) -> Traverser<T> {
        let string_vec: Vec<String> = orig.split(" ")
                                          .filter(|&word| word != "")
                                          .map(|x| x.to_string())
                                          .collect();

        let first_word = string_vec[0].clone();

        let dict_unwrapped = dict.lock().unwrap();

        let mut def: Arc<_> = Arc::new(Mutex::new(VSON::Noun("ERROR".to_string())));
        match dict_unwrapped.get_trav_behavior(&first_word){
            TravBehavior::Verbal => {
                def = Arc::new(Mutex::new(VSON::VSO(first_word, None, None)))
            },
            TravBehavior::Nominal => {
                def = Arc::new(Mutex::new(VSON::VSO(first_word, None, None)))
            },
            _ => {}
        }
        
        Traverser {
            string_vec,
            vec_ptr: 0,
            store: def.clone(),
            store_ptr: def.clone(),
            prev_loc_stack: Vec::new(),
            dict: dict.clone(),
        }
    }

    pub fn pprint(thing: &mut VSON, tab_count: usize) {
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
    }
}

impl<T: TravDict> Traverser<T> {
    pub fn pprint_store(&mut self) {
        Self::pprint(&mut *self.store.lock().unwrap(), 0);
    }

    pub fn pprint_store_ptr(&mut self) {
        Self::pprint(&mut *self.store_ptr.lock().unwrap(), 0);
    }

    pub fn get_next_word(&mut self) -> String {
        self.vec_ptr += 1;
        if self.vec_ptr == self.string_vec.len() {
            "END_OF_FILE".to_string()
        } else if self.vec_ptr > self.string_vec.len() {
            "PAST_END".to_string()
        } else {
            self.string_vec[self.vec_ptr].clone()
        }
    }

    pub fn call_step(&mut self) {
        let next_word = self.get_next_word();
        let next_trav_behavior = self.dict.lock().unwrap().get_trav_behavior(&next_word);
        //println!("{}", next_word);
        self.step(next_word, next_trav_behavior);
    }

    pub fn step(&mut self, next_word: String, next_trav_behavior: TravBehavior) {
        self.pprint_store_ptr();
        /*
        match next_type {
            WordType::END_OF_FILE | WordType::PAST_END => {
                return
            },
            _ => {}
        }
        */
        let mut borrowed_content = self.store_ptr.lock().unwrap();
        match &*borrowed_content {
            VSON::VSO(val, None, maybe_obj) => {
                assert!(maybe_obj.is_none()); // maybe_obj should be None!
                match next_trav_behavior {
                    TravBehavior::END_OF_FILE | TravBehavior::PAST_END => {
                        return
                    },

                    TravBehavior::Nominal => {
                        *borrowed_content = VSON::VSO(val.clone(), Some(Arc::new(Mutex::new(VSON::Noun(next_word)))), maybe_obj.clone());
                    }

                    TravBehavior::Verbal => { // should only go there if it's NOT a noun
                        *borrowed_content = VSON::VSO(val.clone(), Some(Arc::new(Mutex::new(VSON::VSO(next_word, None, None)))), maybe_obj.clone());
                        drop(borrowed_content);
                        if let VSON::VSO(_, Some(thing), _) = &*self.store_ptr.clone().lock().unwrap() {
                            self.prev_loc_stack.push(self.store_ptr.clone());
                            self.store_ptr = thing.clone(); // clone should only increment the ref counter
                        }
                        /*
                        if let VSON::VSO(_, Some(thing), _) = &*borrowed_content {
                            self.store_ptr = thing.clone(); // clone should only increment the ref counter
                        }
                        */
                    }
                }
                
            } // No Subject
            VSON::VSO(val, Some(thing), None) => {
                match next_trav_behavior {
                    TravBehavior::END_OF_FILE | TravBehavior::PAST_END => {
                        return
                    },

                    TravBehavior::Nominal => {
                        *borrowed_content = VSON::VSO(val.clone(), Some(thing.clone()), Some(Arc::new(Mutex::new(VSON::Noun(next_word)))));
                    }

                    TravBehavior::Verbal => {
                        *borrowed_content = VSON::VSO(val.clone(), Some(thing.clone()), Some(Arc::new(Mutex::new(VSON::VSO(next_word, None, None)))));
                        drop(borrowed_content);
                        if let VSON::VSO(_, _, Some(thing)) = &*self.store_ptr.clone().lock().unwrap() {
                            self.prev_loc_stack.push(self.store_ptr.clone());
                            self.store_ptr = thing.clone(); // clone should only increment the ref counter
                        }
                        /*
                        if let VSON::VSO(_, _, Some(thing)) = &*borrowed_content {
                            self.store_ptr = thing.clone(); // clone should only increment the ref counter
                        }
                        */
                    } 
                }
            } // No Object
            VSON::VSO(_, Some(_), Some(_)) => {
                drop(borrowed_content);
                if self.prev_loc_stack.len() > 0 {
                    if let Some(thing) = self.prev_loc_stack.pop() {
                        self.store_ptr = thing;
                    }
                    self.step(next_word, next_trav_behavior);
                    println!("-----COMPLETED VSO CLUSTER-----");
                }
            } // Completed VSO

            VSON::Noun(_) => {
                panic!();
            } // Noun
        }
    }
}

#[derive (Debug)]
pub enum VSON {
    VSO(String, Option<Arc<Mutex<VSON>>>, Option<Arc<Mutex<VSON>>>),
    Noun(String),
}
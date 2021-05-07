use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive (Debug)]
struct Traverser<'a> {
    string_vec: Vec<String>,
    vec_ptr: usize,
    store: Rc<RefCell<VSON>>,
    store_ptr: Rc<RefCell<VSON>>,
    prev_loc_stack: Vec<Rc<RefCell<VSON>>>,
    dict: &'a Dictionary,
}

impl<'a> Traverser<'a> {
    fn new(orig: String, dict: &'a Dictionary) -> Traverser<'a> {
        let string_vec: Vec<String> = orig.split(" ")
                                          .filter(|&word| word != "")
                                          .map(|x| x.to_string())
                                          .collect();

        let first_word = string_vec[0].clone();
        let first_type: WordType = dict.get_type(&first_word);

        let def: Rc<_>;
        match first_type {
            WordType::Noun | WordType::Adj => {
                def = Rc::new(RefCell::new(VSON::Noun(first_word)))
            }
            _ => {
                def = Rc::new(RefCell::new(VSON::VSO(first_word, None, None)))
            }
        }
        
        Traverser {
            string_vec,
            vec_ptr: 0,
            store: def.clone(),
            store_ptr: def.clone(),
            prev_loc_stack: Vec::new(),
            dict,
        }
    }

    fn pprint(thing: &mut VSON, tab_count: usize) {
        let tabs = "\t".repeat(tab_count);
        let tabs_plus_one = tabs.clone() + "\t";
        match thing {
            VSON::VSO(value, Some(subj_vson), Some(obj_vson)) => {
                println!("{tabs}{value}", value=value, tabs=tabs);

                let borrow_subj = &mut *subj_vson.borrow_mut();
                match borrow_subj {
                    VSON::VSO(_,_,_) => {
                        Traverser::pprint(borrow_subj, tab_count + 1);
                    },
                    VSON::Noun(value) => {
                        println!("{tabs_plus_one}{value}", value=value, tabs_plus_one=tabs_plus_one);
                    }
                }

                let borrow_obj = &mut *obj_vson.borrow_mut();
                match borrow_obj {
                    VSON::VSO(_,_,_) => {
                        Traverser::pprint(borrow_obj, tab_count);
                    },
                    VSON::Noun(value) => {
                        println!("{tabs}{value}", value=value, tabs=tabs);
                    }
                }
            },

            VSON::VSO(value, Some(subj_vson), None) => {
                println!("{tabs}{value}", value=value, tabs=tabs);

                let borrow_subj = &mut *subj_vson.borrow_mut();
                match borrow_subj {
                    VSON::VSO(_,_,_) => {
                        Traverser::pprint(borrow_subj, tab_count + 1);
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

impl<'a> Traverser<'a> {
    fn pprint_store(&mut self) {
        Traverser::pprint(&mut *self.store.borrow_mut(), 0);
    }

    fn pprint_store_ptr(&mut self) {
        Traverser::pprint(&mut *self.store_ptr.borrow_mut(), 0);
    }

    fn get_next_word(&mut self) -> String {
        self.vec_ptr += 1;
        if self.vec_ptr == self.string_vec.len() {
            "END_OF_FILE".to_string()
        } else if self.vec_ptr > self.string_vec.len() {
            "PAST_END".to_string()
        } else {
            self.string_vec[self.vec_ptr].clone()
        }
    }

    fn call_step(&mut self) {
        let next_word = self.get_next_word();
        let next_type = self.dict.get_type(&next_word);
        //println!("{}", next_word);
        self.step(next_word, next_type);
    }

    fn step(&mut self, next_word: String, next_type: WordType) {
        match next_type {
            WordType::END_OF_FILE | WordType::PAST_END => {
                return
            },
            _ => {}
        }
        let mut borrowed_content = self.store_ptr.borrow_mut();
        match &*borrowed_content {
            VSON::VSO(val, None, maybe_obj) => {
                assert!(maybe_obj.is_none()); // maybe_obj should be None!
                match next_type {
                    WordType::Noun | WordType::Adj => {
                        *borrowed_content = VSON::VSO(val.clone(), Some(Rc::new(RefCell::new(VSON::Noun(next_word)))), maybe_obj.clone());
                    }

                    _ => { // should only go there if it's NOT a noun
                        *borrowed_content = VSON::VSO(val.clone(), Some(Rc::new(RefCell::new(VSON::VSO(next_word, None, None)))), maybe_obj.clone());
                        drop(borrowed_content);
                        if let VSON::VSO(_, Some(thing), _) = &*self.store_ptr.clone().borrow_mut() {
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
                match next_type {
                    WordType::Noun | WordType::Adj => {
                        *borrowed_content = VSON::VSO(val.clone(), Some(thing.clone()), Some(Rc::new(RefCell::new(VSON::Noun(next_word)))));
                    }

                    _ => {
                        *borrowed_content = VSON::VSO(val.clone(), Some(thing.clone()), Some(Rc::new(RefCell::new(VSON::VSO(next_word, None, None)))));
                        drop(borrowed_content);
                        if let VSON::VSO(_, _, Some(thing)) = &*self.store_ptr.clone().borrow_mut() {
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
                if let Some(thing) = self.prev_loc_stack.pop() {
                    self.store_ptr = thing;
                }
                self.step(next_word, next_type);
                println!("-----COMPLETED VSO CLUSTER-----");
            } // Completed VSO

            VSON::Noun(_) => {
                panic!();
            } // Noun
        }
    }
}

#[derive (Debug)]
enum VSON {
    VSO(String, Option<Rc<RefCell<VSON>>>, Option<Rc<RefCell<VSON>>>),
    Noun(String),
}

#[derive (Debug)]
struct Dictionary {
    dict: HashMap<String, WordType>,
}

impl Dictionary {
    fn new() -> Dictionary{
        let standard_dict: HashMap<String, WordType> = [
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
        
        Dictionary {
            dict: standard_dict,
        }
    }
}

impl Dictionary {
    fn get_type(&self, word: &String) -> WordType {
        match self.dict.get(word) {
            Some(thing) => thing.clone(),
            None => panic!("SUNRISE_ERROR: WordType not assigned to: {}", &word)
        }
    }
}

#[derive (Debug, Clone)]
enum WordType {
    Action,
    Adj,
    Adv,
    Conj,
    Noun,
    END_OF_FILE,
    PAST_END,
}

fn main() {
    let dict1 = Dictionary::new();
    //let mut trav1 = Traverser::new("and and eat and Mike John and apple pear like Mike and Mike and Mike and Mike John Mike".to_string(), &dict1);
    let mut trav1 = Traverser::new("adv. and free in and dignity right equal birth pass. adj. all adj. human being".to_string(), &dict1);
    //println!("{:#?}", trav1.store);
    trav1.pprint_store_ptr();
    for i in 0..20 {
        println!("########################################");
        //println!("{:?}", trav1.prev_loc_stack);
        trav1.call_step();
        //println!("{:?}", trav1.store);
        trav1.pprint_store_ptr();
    }
    //Traverser::pprint(&mut *trav1.store.borrow_mut(), 0);
    println!("########################################");
    println!("########################################");
    println!("########################################");
    trav1.pprint_store();
    println!("Hello, world!");
}
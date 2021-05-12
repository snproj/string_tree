mod trav_dict;
mod word_dict;
mod traverser;
mod vson;
mod trav_actionator;
mod word_actionator;

use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let dict1 = Arc::new(Mutex::new(word_dict::WordDict::new(word_actionator::WordActionator::new())));
    //let mut trav1 = Traverser::new("and and eat and Mike John and apple pear like Mike and Mike and Mike and Mike John Mike".to_string(), &dict1);
    let mut trav1: traverser::Traverser<word_dict::WordDict> = traverser::Traverser::new("adv. and free in and dignity right equal birth pass. adj. all adj. human being".to_string(), dict1);
    //println!("{:#?}", trav1.store);
    //trav1.pprint_store_ptr();
    for _i in 0..20 {
        println!("########################################");
        //println!("{:?}", trav1.prev_loc_stack);
        trav1.call_step();
        //println!("{:?}", trav1.store);
        //trav1.pprint_store_ptr();
    }
    //Traverser::pprint(&mut *trav1.store.lock().unwrap(), 0);
    println!("########################################");
    println!("########################################");
    println!("########################################");
    trav1.pprint_store();
    println!("Hello, world!");

    println!("{:?}", &trav1.invoke_actionator(trav1.store_ptr.clone()));
}
use std::env;
use std::time::Instant;

mod input;
pub use crate::input::input_tools::read_words;

mod containers;
pub use crate::containers::containers_tools::str_2_hashmap;

mod search;
pub use crate::search::search_tools::gen_main;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("not enough arguments!")
    }
    let letters = str_2_hashmap(&args[2]);
    let rf_now = Instant::now();
    let words = read_words(&args[1], &letters);
    println!("read and filter time: {:?}", rf_now.elapsed());
    println!("words filtered: {}", words.len());
    if words.len() >= 5 {
        let s_now = Instant::now();
        let solution = gen_main(&words, &letters);
        println!("solve time: {:?}", s_now.elapsed());
        println!("number of solutions: {}", solution.len());
    }
}

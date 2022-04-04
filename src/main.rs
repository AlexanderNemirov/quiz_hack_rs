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
    let (vecs, words) = read_words(&args[1], &letters);
    println!("read and filter time: {:?}", rf_now.elapsed());
    println!("words filtered: {}", words.len());
    if words.len() >= 5 {
        let s_now = Instant::now();
        let solution = gen_main(&vecs, &letters);
        println!("solve time: {:?}", s_now.elapsed());
        println!("number of solutions: {}", solution.len());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_words1000() {
        let path = String::from("data/word_rus1000.txt");

        let letters_str_1 = String::from("тетачасчастьчетачечет");
        let letters_hash_1 = str_2_hashmap(&letters_str_1);
        let (vecs_1, _) = read_words(&path, &letters_hash_1);
        assert_eq!(6, vecs_1.len());
        assert_eq!(0, gen_main(&vecs_1, &letters_hash_1).len());

        let letters_str_2 = String::from("экономикатеатрновоегорломинистр");
        let letters_hash_2 = str_2_hashmap(&letters_str_2);
        let (vecs_2, _) = read_words(&path, &letters_hash_2);
        assert_eq!(132, vecs_2.len());
        assert_eq!(4, gen_main(&vecs_2, &letters_hash_2).len());

        let letters_str_3 = String::from("данныелесгодцифраозеро");
        let letters_hash_3 = str_2_hashmap(&letters_str_3);
        let (vecs_3, _) = read_words(&path, &letters_hash_3);
        assert_eq!(66, vecs_3.len());
        assert_eq!(3, gen_main(&vecs_3, &letters_hash_3).len());
    }
}

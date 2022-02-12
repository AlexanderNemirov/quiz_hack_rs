use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct WordVecHash {
    word_hash: HashMap<char, u16>,
    word_vec: Vec<(char, u16)>,
}

impl WordVecHash {
    fn merge(&self, other: &WordVecHash) -> WordVecHash {
        let mut merged = WordVecHash { 
            word_hash: self.word_hash.clone(), 
            word_vec: self.word_vec.clone(),
        };
        for (letter, count) in &other.word_hash {
            if merged.word_hash.contains_key(letter) {
                for elem in &mut merged.word_vec {
                    if elem.0 == *letter { elem.1 += count; break }
                }
            }
            else {
                merged.word_hash.insert(*letter, *count);
                merged.word_vec.push((*letter, *count));
            }
        }
        merged
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { 
        panic!("not enough arguments!")
    }
    let letters = str_2_hashmap(&args[2]);
    for (letter, count) in &letters {
        println!("{}: {}", letter, count);
    }
    let words = read_words(&args[1], &letters);
    println!("words filtered: {}", words.len());
}

fn str_2_hashmap(word: &String) -> HashMap<char, u16> {
    let mut word_hashmap: HashMap<char, u16> = HashMap::new();
    for letter in word.chars() {
        let count = word_hashmap.entry(letter).or_insert(0);
        *count += 1;
    }
    word_hashmap
}

fn filter_word(word_hash: &HashMap<char, u16>, 
    letters: &HashMap<char, u16>) -> Option<Vec<(char, u16)>> {
    let mut word_vec: Vec<(char, u16)> = Vec::with_capacity(word_hash.len());
    for (letter, count) in word_hash {
        if letters.contains_key(letter) && *count <= letters[letter] {
            word_vec.push((*letter, *count));
        }
        else { return None }
    }
    Some(word_vec)
}

fn read_words(path: &String, letters: &HashMap<char, u16>) -> Vec<WordVecHash> {
    let file_in = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file_in) => file_in,
    };
    let buff_in = BufReader::new(file_in);
    let mut filter_wrds: Vec<WordVecHash> = Vec::new();
    for line_opt in buff_in.lines() {
        let line = if let Ok(line) = line_opt { line } else { continue };
        let word_hash = str_2_hashmap(&line);
        if let Some(word_vec) = filter_word(&word_hash, letters) {
            let word = WordVecHash{word_hash, word_vec};
            filter_wrds.push(word);
        }
        else { continue; }
    }
    filter_wrds
}

fn gen(words: &Vec<WordVecHash>, letters: &HashMap<char, u16>, 
    indices: &mut Vec<usize>, solution: &mut Vec<[usize; 5]>) {
}

fn gen_main(words: &Vec<WordVecHash>, letters: &HashMap<char, u16>) -> Vec<[usize; 5]> {
    let mut solution: Vec<[usize; 5]> = Vec::new();
    let mut indices: Vec<usize> = Vec::with_capacity(5);
    for index in 0..words.len()-4 {
        indices.push(index);
        gen(words, letters, &mut indices, &mut solution);
        indices.pop();
    }
    solution
}

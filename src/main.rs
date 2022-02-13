use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use std::collections::HashMap;

fn merge_vecs(first: &Vec<(char, u16)>, second: &Vec<(char, u16)>) -> Vec<(char, u16)> {
    let mut merged = first.clone();
    //let map: HashMap<_, _> = merged.clone().into_iter().collect();
    for (letter, count) in second {
        if let Some(elem) = merged.iter_mut().find(|x| x.0 == *letter) {
            elem.1 += count;
        }
      //if map.contains_key(letter) {
      //    for elem in &mut merged {
      //        if elem.0 == *letter { elem.1 += count; break }
      //    }
      //}
        else {
            merged.push((*letter, *count));
        }
    }
    merged
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { 
        panic!("not enough arguments!")
    }
    let letters = str_2_hashmap(&args[2]);
    let rf_now = Instant::now();
    let words = read_words(&args[1], &letters);
    println!("read and filter time: {}", rf_now.elapsed().as_millis());
    println!("words filtered: {}", words.len());
    if words.len() >= 5 {
        let s_now = Instant::now();
        let solution = gen_main(&words, &letters);
        println!("solve time: {}", s_now.elapsed().as_millis());
        println!("number of solutions: {}", solution.len());
    }
}

fn str_2_hashmap(word: &String) -> HashMap<char, u16> {
    let mut word_hashmap: HashMap<char, u16> = HashMap::new();
    for letter in word.chars() {
        let count = word_hashmap.entry(letter).or_insert(0);
        *count += 1;
    }
    word_hashmap
}

fn str_2_vec(word: &String) -> Vec<(char, u16)> {
    let mut word_hashmap: HashMap<char, u16> = HashMap::new();
    for letter in word.chars() {
        let count = word_hashmap.entry(letter).or_insert(0);
        *count += 1;
    }
    word_hashmap.into_iter().collect()
}

fn filter_word(word_vec: &Vec<(char, u16)>, letters: &HashMap<char, u16>) -> bool {
    for (letter, count) in word_vec {
        if !letters.contains_key(letter) || *count > letters[letter] {
            return false
        }
    }
    true
}

fn read_words(path: &String, letters: &HashMap<char, u16>) -> Vec<Vec<(char, u16)>> {
    let file_in = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file_in) => file_in,
    };
    let buff_in = BufReader::new(file_in);
    let mut filter_wrds: Vec<Vec<(char, u16)>> = Vec::new();
    for line_opt in buff_in.lines() {
        let line = if let Ok(line) = line_opt { line } else { continue };
        let word_vec = str_2_vec(&line);
        if filter_word(&word_vec, letters) {
            filter_wrds.push(word_vec);
        }
        else { continue }
    }
    filter_wrds
}

fn final_check(combined: &Vec<(char, u16)>, letters: &HashMap<char, u16>) -> bool {
    if combined.len() != letters.len() { return false }
    for (letter, count) in combined {
        if *count != letters[letter] {
            return false
        }
    }
    true
}

fn iter_check(combined: &Vec<(char, u16)>, letters: &HashMap<char, u16>, len: usize) -> bool {
    if combined.len() > letters.len() { return false }
    for i in 0..len {
        if combined[i].1 > letters[&combined[i].0] {
            return false
        }
    }
    true
}


fn gen(words: &Vec<Vec<(char, u16)>>, letters: &HashMap<char, u16>, combined: &Vec<(char, u16)>,
       index_start: usize, indices: &mut Vec<usize>, solution: &mut Vec<[usize; 5]>) {
    if indices.len() == 5 {
        if final_check(combined, letters) {
            solution.push([indices[0],
                           indices[1],
                           indices[2],
                           indices[3],
                           indices[4]]);
        }
        else { return }
    }
    for index in index_start..words.len() {
        let comb_next = merge_vecs(&words[index], combined);
        if !iter_check(&comb_next, letters, words[index].len()) { continue }
        indices.push(index);
        gen(words, letters, &comb_next, index+1, indices, solution);
        indices.pop();
    }

}

fn gen_main(words: &Vec<Vec<(char, u16)>>, letters: &HashMap<char, u16>) -> Vec<[usize; 5]> {
    let mut solution: Vec<[usize; 5]> = Vec::new();
    let mut indices: Vec<usize> = Vec::with_capacity(5);
    for index in 0..words.len()-4 {
        indices.push(index);
        gen(words, letters, &words[index], index+1, &mut indices, &mut solution);
        indices.pop();
    }
    solution
}

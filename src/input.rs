pub mod input_tools {

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub use crate::containers::containers_tools::str_2_vec;

    fn filter_word(word_vec: &Vec<(char, u16)>, letters: &HashMap<char, u16>) -> bool {
        for (letter, count) in word_vec {
            if !letters.contains_key(letter) || *count > letters[letter] {
                return false;
            }
        }
        true
    }

    pub fn read_words(path: &String, letters: &HashMap<char, u16>) -> Vec<Vec<(char, u16)>> {
        let file_in = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path, why),
            Ok(file_in) => file_in,
        };
        let buff_in = BufReader::new(file_in);
        let mut filter_wrds: Vec<Vec<(char, u16)>> = Vec::new();
        for line_opt in buff_in.lines() {
            let line = if let Ok(line) = line_opt {
                line
            } else {
                continue;
            };
            let word_vec = str_2_vec(&line);
            if filter_word(&word_vec, letters) {
                filter_wrds.push(word_vec);
            } else {
                continue;
            }
        }
        filter_wrds
    }
}

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

    pub fn read_words(
        path: &String,
        letters: &HashMap<char, u16>,
    ) -> (Vec<Vec<(char, u16)>>, Vec<String>) {
        let file_in = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path, why),
            Ok(file_in) => file_in,
        };
        let buff_in = BufReader::new(file_in);
        let mut filter_vecs: Vec<Vec<(char, u16)>> = Vec::new();
        let mut filter_wrds: Vec<String> = Vec::new();
        for line_opt in buff_in.lines() {
            let line = if let Ok(line) = line_opt {
                line
            } else {
                continue;
            };
            let word_vec = str_2_vec(&line);
            if filter_word(&word_vec, letters) {
                filter_vecs.push(word_vec);
                filter_wrds.push(line);
            } else {
                continue;
            }
        }
        (filter_vecs, filter_wrds)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::collections::HashMap;

        #[test]
        fn filter_test() {
            let letters_test = HashMap::from([('w', 1), ('o', 1), ('r', 1), ('d', 1)]);
            let word_test_1 = vec![('d', 1), ('r', 1), ('o', 1), ('w', 1)];
            let word_test_2 = vec![('d', 1), ('r', 1), ('w', 1)];
            let word_test_3 = vec![('d', 1), ('z', 1), ('w', 1)];

            assert_eq!(true, filter_word(&word_test_1, &letters_test));
            assert_eq!(true, filter_word(&word_test_2, &letters_test));
            assert_eq!(false, filter_word(&word_test_3, &letters_test));
        }

        #[test]
        fn read_test() {
            //тетачасчастьчетачечет
            let letters_test =
                HashMap::from([('т', 5), ('а', 4), ('е', 4), ('ч', 5), ('с', 2), ('ь', 1)]);
            let path_test = String::from("data/word_rus1000.txt");

            let words_test = vec!["часть", "час", "счет", "сеть", "честь", "счастье"];
            let (_, words_file) = read_words(&path_test, &letters_test);

            assert_eq!(words_test.len(), words_file.len());
        }
    }
}

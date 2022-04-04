pub mod containers_tools {
    use std::collections::HashMap;

    pub fn str_2_hashmap(word: &String) -> HashMap<char, u16> {
        let mut word_hashmap: HashMap<char, u16> = HashMap::new();
        for letter in word.chars() {
            let count = word_hashmap.entry(letter).or_insert(0);
            *count += 1;
        }
        word_hashmap
    }

    pub fn str_2_vec(word: &String) -> Vec<(char, u16)> {
        str_2_hashmap(&word).into_iter().collect()
    }

    pub fn merge_vecs(first: &Vec<(char, u16)>, second: &Vec<(char, u16)>) -> Vec<(char, u16)> {
        let mut merged = first.clone();
        for (letter, count) in second {
            if let Some(elem) = merged.iter_mut().find(|x| x.0 == *letter) {
                elem.1 += count;
            } else {
                merged.push((*letter, *count));
            }
        }
        merged
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use std::collections::HashMap;

        #[test]
        fn str_2_hashmap_test() {
            let hash_test_1 = HashMap::from([('w', 1), ('o', 1), ('r', 1), ('d', 1)]);
            let word_test_1 = String::from("wrod");
            assert_eq!(hash_test_1, str_2_hashmap(&word_test_1));

            let hash_test_2 = HashMap::from([('s', 1), ('m', 1), ('e', 1), ('l', 2)]);
            let word_test_2 = String::from("smell");
            assert_eq!(hash_test_2, str_2_hashmap(&word_test_2));
        }

        #[test]
        fn merge_vecs_test() {
            let vec_test_1 = vec![('w', 1), ('o', 1), ('r', 1), ('d', 1)];
            let vec_test_2 = vec![('c', 1), ('r', 1), ('o', 1), ('w', 1)];
            let merged_test_12 = vec![('w', 2), ('o', 2), ('r', 2), ('d', 1), ('c', 1)];
            assert_eq!(merged_test_12, merge_vecs(&vec_test_1, &vec_test_2));
        }
    }
}

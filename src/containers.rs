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
        let mut word_hashmap: HashMap<char, u16> = HashMap::new();
        for letter in word.chars() {
            let count = word_hashmap.entry(letter).or_insert(0);
            *count += 1;
        }
        word_hashmap.into_iter().collect()
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
}

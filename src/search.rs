pub mod search_tools {

    use std::collections::HashMap;

    pub use crate::containers::containers_tools::merge_vecs;

    fn final_check(combined: &Vec<(char, u16)>, letters: &HashMap<char, u16>) -> bool {
        if combined.len() != letters.len() {
            return false;
        }
        for (letter, count) in combined {
            if *count != letters[letter] {
                return false;
            }
        }
        true
    }

    fn iter_check(combined: &Vec<(char, u16)>, letters: &HashMap<char, u16>, len: usize) -> bool {
        for i in 0..len {
            if combined[i].1 > letters[&combined[i].0] {
                return false;
            }
        }
        true
    }

    fn gen(
        words: &Vec<Vec<(char, u16)>>,
        letters: &HashMap<char, u16>,
        combined: &Vec<(char, u16)>,
        index_start: usize,
        indices: &mut Vec<usize>,
        solution: &mut Vec<[usize; 5]>,
    ) {
        if indices.len() == 5 {
            if final_check(combined, letters) {
                solution.push([indices[0], indices[1], indices[2], indices[3], indices[4]]);
            } else {
                return;
            }
        }
        for index in index_start..words.len() {
            let comb_next = merge_vecs(&words[index], combined);
            if !iter_check(&comb_next, letters, words[index].len()) {
                continue;
            }
            indices.push(index);
            gen(words, letters, &comb_next, index + 1, indices, solution);
            indices.pop();
        }
    }

    pub fn gen_main(
        words: &Vec<Vec<(char, u16)>>,
        letters: &HashMap<char, u16>,
    ) -> Vec<[usize; 5]> {
        let mut solution: Vec<[usize; 5]> = Vec::new();
        let mut indices: Vec<usize> = Vec::with_capacity(5);
        for index in 0..words.len() - 4 {
            indices.push(index);
            gen(
                words,
                letters,
                &words[index],
                index + 1,
                &mut indices,
                &mut solution,
            );
            indices.pop();
        }
        solution
    }
}

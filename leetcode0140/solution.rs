impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        /// Backtracking
        /// 
        /// Time: O(2^n) | Space: O(n * 2^n)
        /// where n is the length of s
        let mut res: Vec<String> = Vec::new();
        let mut path: Vec<String> = Vec::new();

        // use a vec to  store used words to avoid iterating over the whole word_dict
        // to improve the performance
        let mut used_words: Vec<String> = Vec::new();

        Self::backtracking(&mut res, &mut path, 0, &s, &mut used_words, &word_dict);

        res
        
    }

    fn backtracking(res: &mut Vec<String>, path: &mut Vec<String>, current: usize, s: &String, used_words: &mut Vec<String>, word_dict: &Vec<String>) {
        if current == s.len() {
            let joined = path.join(" ");
            res.push(joined);
        } else {
            for i in current..s.len() {
                let word: String = String::from(&s[current..=i]);

                if used_words.contains(&word) {
                    path.push(word.clone());
                    Self::backtracking(res, path, i + 1, s, used_words, word_dict);
                    path.pop();
                } else if word_dict.contains(&word) {
                    used_words.push(word.clone());
                    path.push(word.clone());
                    Self::backtracking(res, path, i + 1, s, used_words, word_dict);
                    path.pop();
                    used_words.pop();
                }
            }
        }
    }
}
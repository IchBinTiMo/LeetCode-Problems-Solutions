impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.find(ch) {
            Some(idx) => {
                let mut res: String = word[0..=idx].chars().rev().collect::<String>();
                res.push_str(&word[(idx + 1)..]);
                res
            },
            None => word
        }
    }
}
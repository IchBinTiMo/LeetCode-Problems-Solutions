use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut smp: HashMap<char, i32> = HashMap::new();
        let mut tmp: HashMap<char, i32> = HashMap::new();

        for (char_s, char_t) in s.chars().zip(t.chars()) {
            *smp.entry(char_s).or_insert(0) += 1;
            *tmp.entry(char_t).or_insert(0) += 1;
        }
        smp == tmp
    }
}
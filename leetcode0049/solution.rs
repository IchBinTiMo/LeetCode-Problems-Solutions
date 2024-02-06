use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ht: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for s in strs.iter() {
            let mut counter: [u8; 26] = [0; 26];
            for byte in s.bytes() {
                counter[(byte - b'a') as usize] += 1;
            }

            ht.entry(counter).and_modify(|v| v.push(s.to_string())).or_insert(vec![s.to_string()]);
        }

        ht.into_values().collect()
    }
}
use std::collections::HashMap;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let l = words.len() as i32;

        let mut map: HashMap<u8, i32> = HashMap::new();

        for word in words.iter() {
            for byte in word.bytes() {
                *map.entry(byte).or_insert(0) += 1;
            }
        }

        for key in map.keys() {
            let value = map.get(&key).unwrap();
            if value % l != 0 {
                return false;
            }
        }

        true
    }
}
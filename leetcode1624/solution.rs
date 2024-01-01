use std::collections::HashMap;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut map: HashMap<u8, Vec<usize>> = HashMap::new();
        let mut ans = -1;

        for (idx, byte) in s.bytes().enumerate() {
            map.entry(byte).or_insert(Vec::new()).push(idx);
        }

        for key in map.keys() {
            let diff = (map[key].last().unwrap() - map[key][0]) as i32 - 1;
            ans = ans.max(diff);
        }

        ans   
    }
}
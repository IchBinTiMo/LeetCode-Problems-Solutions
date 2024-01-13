use std::collections::HashMap;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut map: HashMap<u8, i32> = HashMap::new();

        for byte in s.bytes() {
            *map.entry(byte).or_insert(0) += 1;
        }


        for byte in t.bytes() {
            if let Some(cnt) = map.get_mut(&byte) {
                *cnt -= 1;
                if *cnt == 0 {
                    map.remove(&byte);
                }
            } 
        }

        map.values().sum::<i32>()
    }
}
use std::collections::HashMap;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        /// Brute force solution
        let mut res: i64 = 0;

        let mut masks: HashMap<i32, i64> = HashMap::new();

        let mut current_mask: i32 = 0;

        masks.insert(0, 1);

        for c in word.chars() {
            let c: i32 = (c as i32) - ('a' as i32);

            current_mask ^= 1 << c;

            for key in masks.keys() {
                if (key ^ current_mask).count_ones() < 2 {
                    res += masks[key];
                }
            }

            masks.entry(current_mask).and_modify(|mask| *mask += 1).or_insert(1);
        }

        res

    }
}
use std::collections::HashMap;

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut res: i64 = 0;

        let mut masks: HashMap<i32, i64> = HashMap::new();

        let mut current_mask: i32 = 0;

        masks.insert(0, 1);

        for c in word.bytes() {
            current_mask ^= 1 << ((c - b'a') as i32);

            // find all masks that is differs from current mask by exactly 1 bit
            // which means current mask ^ these prefix mask == 1
            for i in 0..10 {
                if let Some(&count) = masks.get(&(current_mask ^ (1 << i))) {
                    res += count;
                }
            }

            masks.entry(current_mask).and_modify(|mask| *mask += 1).or_insert(1);

            // current mask ^ this prefix mask == 0
            res += masks[&current_mask] - 1;
        }

        res

    }
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn wonderful_substrings(word: String) -> i64 {
//         /// brute force
//         let mut res: i64 = 0;

//         let mut masks: HashMap<i32, i64> = HashMap::new();

//         let mut current_mask: i32 = 0;

//         masks.insert(0, 1);

//         for c in word.chars() {
//             let c: i32 = (c as i32) - ('a' as i32);

//             current_mask ^= 1 << c;

//             for key in masks.keys() {
//                 if (key ^ current_mask).count_ones() < 2 {
//                     res += masks[key];
//                 }
//             }

//             masks.entry(current_mask).and_modify(|mask| *mask += 1).or_insert(1);
//         }

//         res

//     }
// }
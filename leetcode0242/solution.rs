impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false
        }

        let mut letters = [0; 26];

        for byte in s.bytes() {
            letters[(byte - b'a') as usize] += 1;
        }

        for byte in t.bytes() {
            letters[(byte - b'a') as usize] -= 1;
        }

        letters.iter().all(|&l| l == 0)
    }
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn is_anagram(s: String, t: String) -> bool {
//         if s.len() != t.len() {
//             return false
//         }

//         let mut tmp: HashMap<u8, i32> = HashMap::new();

//         for byte in s.bytes() {
//             *tmp.entry(byte).or_insert(0) += 1;
//         }

//         for byte in t.bytes() {
//             *tmp.entry(byte).or_insert(0) -= 1;
//             if let Some(val) = tmp.get(&byte) {
//                 if *val == 0 {
//                     tmp.remove(&byte);
//                 }
//             }
//         }

//         tmp.values().len() == 0
//     }
// }

// // use std::collections::HashMap;

// // impl Solution {
// //     pub fn is_anagram(s: String, t: String) -> bool {
// //         if s.len() != t.len() {
// //             return false
// //         }

// //         let mut smp: HashMap<char, i32> = HashMap::new();
// //         let mut tmp: HashMap<char, i32> = HashMap::new();

// //         for (char_s, char_t) in s.chars().zip(t.chars()) {
// //             *smp.entry(char_s).or_insert(0) += 1;
// //             *tmp.entry(char_t).or_insert(0) += 1;
// //         }
// //         smp == tmp
// //     }
// // }
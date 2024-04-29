impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let n: usize = s.len();

        let mut max_len: Vec<i32> = vec![0; 26];

        for current in s.chars().map(|c| (c as i32) - ('a' as i32)) {

            let mut prev_max: i32 = ((current - k).max(0)..=((current + k).min(25))).fold(i32::MIN, |acc, prev| acc.max(max_len[prev as usize]));

            max_len[current as usize] = prev_max + 1;
        }

        *max_len.iter().max().unwrap()
    }
}

// impl Solution {
//     pub fn longest_ideal_string(s: String, k: i32) -> i32 {
//         let n: usize = s.len();

//         let s: Vec<char> = s.chars().collect::<Vec<char>>();
//         let mut max_len: Vec<i32> = vec![0; 26];

//         for i in 0..n {
//             let current: i32 = (s[i] as u8 - b'a') as i32;

//             for prev in current..=(current + k).min(25) {
//                  max_len[current as usize] = max_len[current as usize].max(max_len[prev as usize] + 1);
//             }

//             for prev in (current - k).max(0)..current {
//                  max_len[current as usize] = max_len[current as usize].max(max_len[prev as usize] + 1);
//             }
//         }

//         *max_len.iter().max().unwrap()
//     }
// }
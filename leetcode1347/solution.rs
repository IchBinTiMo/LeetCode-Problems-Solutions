
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {

        let mut c1: [i32; 26] = [0; 26];

        for byte in s.bytes() {
            c1[(byte - b'a') as usize] += 1;
        }

        for byte in t.bytes() {
            if c1[(byte - b'a') as usize] > 0 {
                c1[(byte - b'a') as usize] -= 1;
            }
        }

        c1.iter().sum()
    }
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn min_steps(s: String, t: String) -> i32 {
//         let mut map: HashMap<u8, i32> = HashMap::new();

//         for byte in s.bytes() {
//             *map.entry(byte).or_insert(0) += 1;
//         }


//         for byte in t.bytes() {
//             if let Some(cnt) = map.get_mut(&byte) {
//                 *cnt -= 1;
//                 if *cnt == 0 {
//                     map.remove(&byte);
//                 }
//             } 
//         }

//         map.values().sum::<i32>()
//     }
// }
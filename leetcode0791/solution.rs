impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
       let mut occurences: [i32; 26] = [0; 26];

       for byte in s.bytes() {
           occurences[(byte - b'a') as usize] += 1;
       }

       let mut res: Vec<char> = Vec::new();

       for byte in order.bytes() {
           let idx: usize = (byte - b'a') as usize;
           while occurences[idx] > 0 {
               res.push(char::from(byte));
               occurences[idx] -= 1;
           }
       }

       for i in 0..26 {
           while occurences[i] > 0 {
               res.push(char::from((i as u8) + b'a'));
               occurences[i] -= 1;
           }
       }

       res.iter().collect()
    }
}

// impl Solution {
//     pub fn custom_sort_string(order: String, s: String) -> String {
        
//         let mut indices: [usize; 26] = [usize::MAX; 26];

//         for (idx, byte) in order.bytes().enumerate() {
//             indices[(byte - b'a') as usize] = idx;
//         }

//         let mut bytes: Vec<u8> = s.bytes().collect();

        
//         bytes.sort_unstable_by_key(|byte| indices[(byte - b'a') as usize]);

//         bytes.iter().map(|&byte| char::from(byte)).collect()
//     }
// }
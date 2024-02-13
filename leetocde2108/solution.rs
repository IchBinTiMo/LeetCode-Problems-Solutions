impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {

        for word in words.iter() {
            let mut cnt: usize = 0;
            let mut palin: bool = true;

            for (left, right) in word.bytes().zip(word.bytes().rev()) {
                if left != right {
                    palin = false;
                    break;
                }
                cnt += 1;
                if cnt > word.len() / 2 {
                    break;
                }
            }

            if palin {
                return word.to_string();
            }
        }

        String::new()
    }
}

// impl Solution {
//     pub fn first_palindrome(words: Vec<String>) -> String {
//         for word in words.iter() {

//             let bytes: Vec<u8> = word.bytes().collect();

//             let len = bytes.len();

//             let mut palin: bool = true;

//             if len & 1 == 1 {
//                 for i in 1..=(len / 2) {
//                     if bytes[len / 2 - i] != bytes[len / 2 + i] {
//                         palin = false;
//                         break;
//                     }
//                 }
//                 if palin {
//                     return word.to_string();
//                 }
//             } else {
//                 for i in 0..(len / 2) {
//                     if bytes[i] != bytes[len - i - 1] {
//                         palin = false;
//                         break;
//                     }
//                 }
//                 if palin {
//                     return word.to_string();
//                 }
//             }
//         }

//         String::new()
//     }
// }
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        /// Time O(n) | Space O(1)
        /// where n is the length of the string
        let mut freqs: [i32; 58] = [0; 58];

        for c in s.chars() {
            freqs[(c as usize) - 65] += 1;
        }

        // there is at most one odd frequency in a palindrome
        // false means the frequencies of all characters in the palindrome are even
        let mut one: bool = false;

        let mut res: i32 = 0;

        for freq in freqs {
            if one {
                // there is one character with odd frequency in the palindrome
                // so we can't have more than one character with odd frequency
                res += freq / 2 * 2;
            } else {
                res += freq;
                if freq & 1 == 1 {
                    one = true;
                }
            }
        }

        res
    }
}

// impl Solution {
//     pub fn longest_palindrome(s: String) -> i32 {
//         /// Time O(n) | Space O(1)
//         /// where n is the length of the string
//         let mut freqs: [i32; 58] = [0; 58];

//         for c in s.chars() {
//             freqs[(c as usize) - 65] += 1;
//         }

//         // there is at most one odd frequency in a palindrome
//         // false means the frequencies of all characters in the palindrome are even
//         let mut one: bool = false;

//         let mut res: i32 = 0;

//         for freq in freqs {
//             if freq == 0 {
//                 continue;
//             }

//             if one {
//                 // there is one character with odd frequency in the palindrome
//                 // so we can't have more than one character with odd frequency
//                 res += freq / 2 * 2;
//             } else {
//                 res += freq;
//                 if freq & 1 == 1 {
//                     one = true;
//                 }
//             }
//         }

//         res
//     }
// }
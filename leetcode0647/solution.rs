impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut ans: i32 = 0;

        let s: Vec<u8> = s.bytes().collect();

        for i in 0..s.len() {
            ans += Solution::expand(&s, i, i) + Solution::expand(&s, i, i + 1);
        }

        ans
    }

    fn expand(s: &Vec<u8>, mut left: usize, mut right: usize) -> i32 {
        let mut ans: i32 = 0;

        let n: usize = s.len();

        while left < n && right < n && s[left] == s[right] {
            left -= 1;
            right += 1;
            ans += 1;
        }

        ans
    }
}

// impl Solution {
//     pub fn count_substrings(s: String) -> i32 {
//         if s.len() == 1 {
//             return 1;
//         }

//         let mut ans: i32 = 0;

//         let s: Vec<u8> = s.bytes().collect();

//         let n: usize = s.len();

//         for i in 0..n {
//             let mut left: usize = i;
//             let mut right: usize = i;

//             while left < n && right < n && s[left] == s[right] {
//                 left -= 1;
//                 right += 1;
//                 ans += 1;
//             }

//             left = i;
//             right = i + 1;

//             while left < n && right < n && s[left] == s[right] {
//                 ans += 1;
//                 left -= 1;
//                 right += 1;
//             }
//         }

//         ans
//     }
// }

// // impl Solution {
// //     pub fn count_substrings(s: String) -> i32 {
// //         if s.len() == 1 {
// //             return 1;
// //         }

// //         let mut ans: i32 = 0;

// //         let s: Vec<u8> = s.bytes().collect();

// //         let n: usize = s.len();

// //         for i in 0..n {
// //             let mut left: usize = i;
// //             let mut right: usize = i;

// //             ans += 1;

// //             while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
// //                 left -= 1;
// //                 right += 1;
// //                 ans += 1;
// //             }

// //             left = i;
// //             right = i + 1;

// //             while left < n && right < n && s[left] == s[right] {
// //                 ans += 1;
// //                 left -= 1;
// //                 right += 1;
// //             }
// //         }

// //         ans
// //     }
// // }
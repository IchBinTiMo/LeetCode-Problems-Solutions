impl Solution {
    pub fn max_score(s: String) -> i32 {
        let l = s.len();

        let mut ones = s.bytes().filter(|&byte| byte == 49).count();

        if ones == l || ones == 0 {
            return l as i32 - 1;
        }

        let mut zeroes = 0;

        let mut ans = 0;

        for byte in s[0..l - 1].bytes() {
            match byte {
                48 => zeroes += 1,
                _ => ones -= 1
            }

            ans = ans.max(zeroes + ones);
        }

        ans as i32
    }
}

// impl Solution {
//     pub fn max_score(s: String) -> i32 {

//         if !s.contains('0') || !s.contains('1') {
//             return s.len() as i32- 1;
//         }

//         let mut current = 0;

//         for byte in s.bytes() {
//             match byte {
//                 49 => current += 1,
//                 _ => {}
//             }
//         }

//         let mut ans = 0;

//         for (i, byte) in s.bytes().enumerate() {
//             if i == s.len() - 1 {
//                 continue;
//             }
//             match byte {
//                 48 => current += 1,
//                 _ => current -= 1
//             }
//             ans = ans.max(current);
//         }

//         ans
//     }
// }
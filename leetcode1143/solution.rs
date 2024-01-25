impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {

        let bytes1: Vec<u8> = text1.bytes().collect::<Vec<u8>>();
        let bytes2: Vec<u8> = text2.bytes().collect::<Vec<u8>>();

        let m: usize = text1.len();
        let n: usize = text2.len();
        let mut prev: Vec<i32> = vec![0; (n + 1)];

        for i in 0..m {
            let mut dp: Vec<i32> = vec![0; (n + 1)];
            for j in 0..n {
                if bytes1[i] == bytes2[j] {
                    dp[j + 1] = prev[j] + 1;
                } else {
                    dp[j + 1] = dp[j].max(prev[j + 1]);
                }
            }
            prev = dp;
        }

        prev[n]
    }
}

// impl Solution {
//     pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {

//         let bytes1: Vec<u8> = text1.bytes().collect::<Vec<u8>>();
//         let bytes2: Vec<u8> = text2.bytes().collect::<Vec<u8>>();

//         let m: usize = text1.len();
//         let n: usize = text2.len();
//         let mut dp: Vec<Vec<i32>> = vec![vec![0; (n + 1)]; (m + 1)];

//         for i in 1..(m + 1) {
//             for j in 1..(n + 1) {
//                 if bytes1[i - 1] == bytes2[j - 1] {
//                     dp[i][j] = dp[i - 1][j - 1] + 1;
//                 } else {
//                     dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
//                 }
//             }
//         }

//         dp[m][n]
//     }
// }
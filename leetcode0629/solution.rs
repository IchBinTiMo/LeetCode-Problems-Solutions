impl Solution {
    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        let mut ans = 0;

        let mut prev: Vec<i32> = vec![0; (k + 1) as usize];

        prev[0] = 1;

        for i in 2..=n {
            let mut dp: Vec<i32> = vec![0; (k + 1) as usize];
            dp[0] = 1;
            for j in 1..=k {
                for x in (j - i + 1).max(0)..=j {
                    let x = x as usize;
                    let j = j as usize;
                    dp[j] = (dp[j] + prev[x]) % 1_000_000_007;
                }
            }
            prev = dp;
        }


        *prev.last().unwrap()
    }
}

// impl Solution {
//     pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
//         let mut ans = 0;

//         let mut prev: Vec<i32> = vec![0; (k + 1) as usize];

//         prev[0] = 1;

//         for i in 2..=n {
//             let mut dp: Vec<i32> = vec![0; (k + 1) as usize];
//             for j in 0..=k {
//                 for x in (j - i + 1).max(0)..=j {
//                     let x = x as usize;
//                     let j = j as usize;
//                     dp[j] = (dp[j] + prev[x]) % 1_000_000_007;
//                 }
//             }
//             prev = dp;
//         }


//         *prev.last().unwrap()
//     }
// }
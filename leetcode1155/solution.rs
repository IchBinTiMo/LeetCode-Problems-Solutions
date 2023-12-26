impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut memo: Vec<Vec<i32>> = vec![vec![-1; (target + 1) as usize]; (n + 1) as usize];
        
        Solution::recursive(&mut memo, n as usize, k as usize, target as usize) as i32
    }

    pub fn recursive(memo: &mut Vec<Vec<i32>>, n: usize, k: usize, target: usize) -> i32 {
        let MOD: i32 = 1_000_000_007;

        if target < n || target > n * k {
            return 0;
        }

        if target == n || target == n * k {
            return 1;
        }

        if memo[n][target] != -1 {
            return memo[n][target] % MOD;
        }

        let mut ans: i32 = 0;

        for i in 1..=k {
            ans = (ans + Solution::recursive(memo, n - 1, k, target - i)) % MOD;
        }

        memo[n][target] = ans % MOD;

        return memo[n][target];

    }
}

// impl Solution {
//     pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
//         let mut dp: Vec<Vec<i32>> = vec![vec![-1; (target + 1) as usize]; (n + 1) as usize];
        
//         Solution::recursive(&mut dp, n as usize, k as usize, target as usize) as i32
//     }

//     pub fn recursive(dp: &mut Vec<Vec<i32>>, n: usize, k: usize, target: usize) -> i32 {
//         let MOD: i32 = 1_000_000_007;

//         if target < n || target > n * k {
//             return 0;
//         }

//         if target == n || target == n * k {
//             return 1;
//         }

//         if dp[n][target] != -1 {
//             return dp[n][target] % MOD;
//         }

//         let mut ans: i32 = 0;

//         for i in 1..=k {
//             ans = (ans + Solution::recursive(dp, n - 1, k, target - i)) % MOD;
//         }

//         dp[n][target] = ans % MOD;

//         return dp[n][target];

//     }
// }

// // impl Solution {
// //     pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
// //         let mut dp: Vec<Vec<i64>> = vec![vec![-1; (target + 1) as usize]; (n + 1) as usize];
        
// //         Solution::recursive(&mut dp, n as usize, k as usize, target as usize) as i32
// //     }

// //     pub fn recursive(dp: &mut Vec<Vec<i64>>, n: usize, k: usize, target: usize) -> i64 {
// //         let MOD: i64 = 1_000_000_007;

// //         if target < n || target > n * k {
// //             return 0;
// //         }

// //         if target == n || target == n * k {
// //             return 1;
// //         }

// //         if dp[n][target] != -1 {
// //             return dp[n][target] % MOD;
// //         }

// //         let mut ans: i64 = 0;

// //         for i in 1..=k {
// //             ans = (ans + Solution::recursive(dp, n - 1, k, target - i)) % MOD;
// //         }

// //         dp[n][target] = ans % MOD;

// //         return dp[n][target];

// //     }
// // }

// // // impl Solution {
// // //     pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
// // //         let mut dp: Vec<Vec<i128>> = vec![vec![-1; (target + 1) as usize]; (n + 1) as usize];
        
// // //         Solution::recursive(&mut dp, n as usize, k as usize, target as usize) as i32
// // //     }

// // //     pub fn recursive(dp: &mut Vec<Vec<i128>>, n: usize, k: usize, target: usize) -> i128 {
// // //         let MOD: i128 = 1_000_000_007;
        
// // //         if target < n || target > n * k {
// // //             return 0;
// // //         }

// // //         if target == n || target == n * k {
// // //             return 1;
// // //         }

// // //         if dp[n][target] != -1 {
// // //             return dp[n][target] % MOD;
// // //         }

// // //         let mut ans: i128 = 0;

// // //         for i in 1..=k {
// // //             ans = (ans + Solution::recursive(dp, n - 1, k, target - i)) % MOD;
// // //         }

// // //         dp[n][target] = ans % MOD;

// // //         return dp[n][target];

// // //     }
// // // }

// // // // impl Solution {
// // // //     pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
// // // //         let mut dp: Vec<Vec<i128>> = vec![vec![-1; (target + 1) as usize]; (n + 1) as usize];
        
// // // //         Solution::recursive(&mut dp, n as usize, k as usize, target as usize) as i32
// // // //     }

// // // //     pub fn recursive(dp: &mut Vec<Vec<i128>>, n: usize, k: usize, target: usize) -> i128 {
// // // //         if target < n || target > n * k {
// // // //             return 0;
// // // //         }

// // // //         if target == n || target == n * k {
// // // //             return 1;
// // // //         }

// // // //         if dp[n][target] != -1 {
// // // //             return dp[n][target] % 1_000_000_007;
// // // //         }

// // // //         let mut ans: i128 = 0;

// // // //         for i in 1..=k {
// // // //             ans = (ans + Solution::recursive(dp, n - 1, k, target - i)) % 1_000_000_007;
// // // //         }

// // // //         dp[n][target] = ans % 1_000_000_007;

// // // //         return dp[n][target];

// // // //     }
// // // // }
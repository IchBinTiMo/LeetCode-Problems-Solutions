impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut dp: Vec<Vec<i128>> = vec![vec![-1; (target + 1) as usize]; (n + 1) as usize];
        
        Solution::recursive(&mut dp, n as usize, k as usize, target as usize) as i32
    }

    pub fn recursive(dp: &mut Vec<Vec<i128>>, n: usize, k: usize, target: usize) -> i128 {
        if target < n || target > n * k {
            return 0;
        }

        if target == n || target == n * k {
            return 1;
        }

        if dp[n][target] != -1 {
            return dp[n][target] % 1_000_000_007;
        }

        let mut ans: i128 = 0;

        for i in 1..=k {
            ans = (ans + Solution::recursive(dp, n - 1, k, target - i)) % 1_000_000_007;
        }

        dp[n][target] = ans % 1_000_000_007;

        return dp[n][target];

    }
}
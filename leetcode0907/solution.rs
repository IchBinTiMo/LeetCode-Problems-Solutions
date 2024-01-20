use std::collections::VecDeque;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_007;

        let mut indices: VecDeque<usize> = VecDeque::new();
        let mut dp: Vec<i32> = vec![0; arr.len()];

        for i in 0..arr.len() {
            while let Some(&idx) = indices.back() {
                if arr[i] < arr[idx] {
                    indices.pop_back();
                } else {
                    break;
                }

            }

            if let Some(&j) = indices.back() {
                dp[i] = dp[j] + ((i - j) as i32) * arr[i];
            } else {
                dp[i] = (i + 1) as i32 * arr[i];
            }

            indices.push_back(i);
        }

        dp.iter().fold(0, |acc, val| (acc + val) % MOD) % MOD
    }
}
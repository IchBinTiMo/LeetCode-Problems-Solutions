use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<HashMap<i64, i32>> = vec![HashMap::new(); nums.len()];

        let mut ans: i32 = 0;

        for i in 1..nums.len() {
            for j in 0..i {
                let diff: i64 = nums[i] as i64 - nums[j] as i64;

                *dp[i].entry(diff).or_insert(0) += 1;

                if let Some(&cnt) = dp[j].get(&diff) {
                    *dp[i].entry(diff).or_insert(0) += cnt;
                    ans += cnt;
                }
            }
        }

        ans
    }
}
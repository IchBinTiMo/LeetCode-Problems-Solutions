impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let n = nums.len();

        let mut dp: Vec<(usize, i32)> = vec![(n + 1, 0); n];

        for i in (0..n).rev() {
            for j in (i + 1)..n {
                if nums[j] % nums[i] == 0 {
                    if dp[j].1 + 1 > dp[i].1 {
                        dp[i] = (j, dp[j].1 + 1);
                    }
                }
            }
        }

        let mut ans: Vec<i32> = Vec::new();

        let mut current: usize = 0;

        let mut l: i32 = i32::MIN;

        for i in 0..n {
            if dp[i].1 > l {
                l = dp[i].1;
                current = i;
            }
        }

        while current < n {
            ans.push(nums[current]);
            current = dp[current].0;
        }

        ans


    }
}
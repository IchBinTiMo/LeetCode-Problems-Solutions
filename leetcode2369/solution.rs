impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut dp: Vec<bool> = vec![true, false, nums[0] == nums[1]];

        for i in 2..nums.len() {
            let mut current: bool = false;
            if nums[i] == nums[i - 1] && dp[1]{
                current = true;
            } else if nums[i] == nums[i - 1] && nums[i] == nums[i - 2] && dp[0] {
                current = true;
            } else if nums[i] == nums[i - 1] + 1 && nums[i] == nums[i - 2] + 2 && dp[0] {
                current = true;
            }
            
            dp[0] = dp[1];
            dp[1] = dp[2];
            dp[2] = current;
        }
        dp[2]
    }
}
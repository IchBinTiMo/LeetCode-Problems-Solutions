// Solution 1
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = 1;

        let mut ans: i32 = 0;

        while right < nums.len() {
            let diff = nums[right] - nums[left];

            let mut cnt: i32 = 0;

            loop {
                right += 1;

                if right >= nums.len() {
                    break;
                }

                if nums[right] - nums[right - 1] != diff {
                    break;
                }

                cnt += 1;
            }

            if cnt > 0 {
                ans += (1..=cnt).sum::<i32>();
            }

            left = right - 1;
        }
        ans
    }
}

// Solution 2
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ans: i32 = 0;

        let mut dp: Vec<i32> = vec![0; nums.len()];

        for i in 2..nums.len() {
            if nums[i] + nums[i - 2] == nums[i - 1] * 2 {
                dp[i] = dp[i - 1] + 1;
            }

            ans += dp[i];
        }
        ans
    }
}
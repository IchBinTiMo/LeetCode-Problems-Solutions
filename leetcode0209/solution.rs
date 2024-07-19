impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        /// Sliding window
        /// 
        /// Time: O(n) | Space: O(1)
        /// 
        /// n - length of nums
        let mut res: i32 = i32::MAX;

        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut sum: i32 = 0;

        while right < nums.len() {
            while right < nums.len() && sum < target {
                sum += nums[right];
                right += 1;
            }

            while left < right && sum >= target {
                res = res.min((right - left) as i32); // update the result
                sum -= nums[left];
                left += 1;
            }
        }

        return if res == i32::MAX {
            0
        } else {
            res
        };
    }
}
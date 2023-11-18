impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        let mut ans: i32 = 1;
        let mut left: usize = 0;
        let mut right: usize = 1;

        nums.sort_unstable();

        let mut tmp: i32 = 0;

        while right < nums.len() {
            tmp += (nums[right] - nums[right - 1]) * ((right - left) as i32);
            while tmp > k {
                tmp -= nums[right] - nums[left];
                left += 1;
            }
            ans = ans.max((right - left + 1) as i32);
            right += 1;
        }
        ans
    }
}
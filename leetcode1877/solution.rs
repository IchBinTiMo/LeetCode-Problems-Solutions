impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut nums: Vec<i32> = nums;
        nums.sort_unstable();
        let mut ans = 0;
        for i in 0..(nums.len() / 2) {
            ans = ans.max(nums[i] + nums[nums.len() - 1 - i]);
        }
        ans
    }
}
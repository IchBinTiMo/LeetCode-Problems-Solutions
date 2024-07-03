impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        /// Time: O(n log n) | Space: O(1)
        /// where n is the length of nums
        if nums.len() <= 4 {
            return 0;
        }
        let n: usize = nums.len();
        let mut res: i32 = i32::MAX;

        nums.sort_unstable();

        for i in 0..4 {
            res = res.min(nums[i + n - 4] - nums[i]);
        }

        res
    }
}
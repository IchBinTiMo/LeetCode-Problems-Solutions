impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        /// Backtracking
        /// 
        /// Time: O(2^N) | Space: O(1)
        let mut res: i32 = 0;

        Self::backtracking(&mut res, 0, 0, &nums);

        res
    }

    fn backtracking(res: &mut i32, path: i32, current: usize, nums: &Vec<i32>) {
        *res += path;

        for i in current..nums.len() {
            Self::backtracking(res, path ^ nums[i], i + 1, nums);
        }
    }
}
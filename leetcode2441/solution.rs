impl Solution {
    pub fn find_max_k(mut nums: Vec<i32>) -> i32 {
        /// Two Pointers
        /// 
        /// Time: O(n log n) | Space: O(1)
        nums.sort_unstable();

        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            if nums[left] == -nums[right] {
                return nums[right];
            } else if nums[left] < -nums[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        -1
    }
}
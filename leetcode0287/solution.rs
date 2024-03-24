impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        /// Binary Search
        /// 
        /// Time Complexity:     O(NlogN)
        /// 
        /// Space Complexity:    O(1)
        /// 
        /// Key: Treat the unsorted array as a sorted array
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let mut count: usize = 0;
            let mid: usize = left + (right - left) / 2;

            // Count the number of elements that are less than or equal to mid
            for i in 0..nums.len() {
                if nums[i] <= mid as i32 {
                    count += 1;
                }
            }

            // If count is more than mid, then we need to search in the left part
            if count > mid {
                right = mid;
            } else { // If count is less than or equal to mid, then we need to search in the right part
                left = mid + 1;
            }
        }

        left as i32
    }
}
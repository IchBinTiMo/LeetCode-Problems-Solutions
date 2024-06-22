impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        /// Store the indices of odd numbers
        /// then slide the window
        /// 
        /// Time: O(n) | Space: O(n)
        /// where n is the length of nums
        let n: usize = nums.len();

        let mut indices: Vec<i32> = Vec::new();

        indices.push(-1); // left bound

        for i in 0..n {
            if nums[i] % 2 == 1 {
                indices.push(i as i32);
            }
        }

        indices.push(n as i32); // right bound

        if indices.len() == 2 {
            // there are no odd numbers
            return 0;
        }

        let mut left: usize = 1;

        let mut res: i32 = 0;

        // 'left' and 'right' both start from 1 
        // because there wont be a number with index -1
        for right in 1..(indices.len() - 1) {
            if right - left + 1 == k as usize {
                // we have k odd numbers
                // so we can form a subarray
                // and shrink the window
                res += (indices[left] - indices[left - 1]) * (indices[right + 1] - indices[right]);
                left += 1;
            }
        }

        res
    }
}
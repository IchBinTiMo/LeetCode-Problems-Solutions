
impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        /// Slide Window
        /// 
        /// Time: O(n) | Space: O(1)
        /// where n is the length of nums
        let n: usize = nums.len();

        let k: usize = k as usize;

        let mut res: i32 = 0;

        let mut is_flipped: i32 = 0; // whether the current window is flipped

        for right in 0..n {
            if right >= k && nums[right - k] == 2 {
                // leave the leftmost window
                // and subtract flipping times by 1
                // which is the same as toggle 'is_flipped'
                is_flipped ^= 1;
            }

            if nums[right] == is_flipped {
                if right + k > n {
                    // if the window is out of bounds
                    // which means we can't flip any more
                    // so return -1
                    return -1;
                }
                nums[right] = 2;
                // increase flipping times by 1
                // which is the same as toggle 'is_flipped'
                is_flipped ^= 1;
                res += 1
            }
        }

        res
    }
}
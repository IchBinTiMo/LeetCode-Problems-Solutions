use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        /// Sliding window & Priority Queue
        /// 
        /// Time: O(n) | Space: O(n)
        /// where n is the length of nums
        let n: usize = nums.len();

        let mut left: usize = 0;

        let mut res: usize = usize::MIN;

        let mut inc: VecDeque<usize> = VecDeque::new(); // min queue
        let mut dec: VecDeque<usize> = VecDeque::new(); // max queue

        for right in 0..n {
            // update the max and min queue
            while let Some(&val) = inc.back() {
                if nums[val] > nums[right] {
                    inc.pop_back();
                } else {
                    break;
                }
            }
            inc.push_back(right);

            while let Some(&val) = dec.back() {
                if nums[val] < nums[right] {
                    dec.pop_back();
                } else {
                    break;
                }
            }
            dec.push_back(right);

            // by now, inc[0] is the smallest value in the window [left, right]
            // and dec[0] is the largest value in the window [left, right]
            while !inc.is_empty() && !dec.is_empty() {
                if (nums[inc[0]] - nums[dec[0]]).abs() <= limit {
                    // update the max subarray length
                    res = res.max(right - left + 1);
                    break;
                } else {
                    // shrink the window
                    left += 1;
                    if inc[0] < left {
                        inc.pop_front();
                    }

                    if dec[0] < left {
                        dec.pop_front();
                    }
                }
            }
        }

        res as i32
    }
}
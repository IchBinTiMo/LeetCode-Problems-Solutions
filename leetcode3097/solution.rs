/*
Solution: Bit Manipulation + Sliding Window

Time: O(n) | Space: O(1)

Runtime: 11 ms | 100.00%
Memory: 3.96 MB | 100.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut res: usize = usize::MAX;

        let mut bits: [i32; 32] = [0; 32];

        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut acc: i32 = 0;

        loop {

            while left < right && acc >= k {
                res = res.min(right - left);

                let mut current: i32 = nums[left];

                let mut i: usize = 0;
                while current > 0 {
                    if current & 1 == 1 {
                        if bits[i] == 1 {
                            acc ^= 1 << i;
                        }

                        bits[i] -= 1;
                    }

                    i += 1;
                    current >>= 1;
                }
                left += 1;
            }

            if right == nums.len() {
                break;
            }

            let mut current: i32 = nums[right];

            let mut i: usize = 0;

            while current > 0 {
                if current & 1 == 1 {
                    bits[i] += 1;
                }

                i += 1;
                current >>= 1;
            }

            acc |= nums[right];
            right += 1;
        }

        return if res == 0 {
            -1
        } else {
            res as i32
        }
    }
}
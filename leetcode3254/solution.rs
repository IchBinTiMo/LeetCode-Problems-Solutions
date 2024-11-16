/*
Solution: Sliding Window

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.26 MB | 12.50%

- n: length of 'nums'
*/

impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k: usize = k as usize;

        if nums.len() == 1 {
            return vec![nums[0]];
        }

        if k == 1 {
            return nums;
        }

        let mut left: usize = 0;
        let mut right: usize = 1;

        let mut res: Vec<i32> = Vec::new();

        let mut valid: bool = true;

        while right < nums.len() {

            if nums[right] - nums[right - 1] != 1 {
                valid = false;
            }
            
            if valid {
                if right - left >= k - 1 {
                    res.push(nums[right]);
                }
            } else {
                if right - left >= k - 1 {
                    left = right - k + 1;
                }
                while left < right && left + k - 1 < nums.len() {
                    res.push(-1);
                    left += 1;
                }
                valid = true
            }

            right += 1;
        }

        res

    }
}
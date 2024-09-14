/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 4 ms | 100.00%
Memory: 4.02 MB | 46.67%

- n: length of 'nums'
*/

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        let mut maxi: i32 = nums[0];
        let mut max_cnt: i32 = 1;
        let mut cnt: i32 = 1;

        for i in 1..n {
            let num: i32 = nums[i];

            if num > maxi {
                maxi = num;
                max_cnt = 1;
                cnt = 1;
            } else if num < maxi {
                cnt = 1;
            }

            if num == maxi && num == nums[i - 1] {
                cnt += 1;
                max_cnt = max_cnt.max(cnt);
            }
        }

        max_cnt
    }
}
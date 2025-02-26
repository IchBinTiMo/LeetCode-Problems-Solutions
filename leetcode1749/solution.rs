/*
Solution: DP, Prefix Sum

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 3.13 MB | 25.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut res: (i32, i32) = (0, 0);

        let mut current: i32 = 0;
        let mut mx: i32 = 0;
        let mut mn: i32 = 0;

        for &num in nums.iter() {
            current += num;

            mx = mx.max(current);
            mn = mn.min(current);

            res.0 = res.0.max(current - mn);
            res.1 = res.1.min(current - mx);
        }

        return if res.0 + res.1 < 0 {
            -res.1
        } else {
            res.0
        };
    }
}
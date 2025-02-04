/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.28 MB | 45.45%

- n: length of nums
*/

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut current: i32 = 0;
        let mut res: i32 = 0;

        let mut q: Vec<i32> = Vec::new();

        for &num in nums.iter() {
            if !q.is_empty() && *q.last().unwrap() >= num {
                q.clear();
                current = 0;
            }

            q.push(num);

            current += num;

            res = res.max(current);
        }

        res
    }
}
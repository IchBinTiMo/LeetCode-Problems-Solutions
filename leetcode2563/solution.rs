/*
Solution: Binary Search

Time: O(n log n) | Space: O(1)

Runtime: 25 ms | 100.00%
Memory: 3.58 MB | 66.67%

- n: length of 'nums'
*/

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut res: i64 = 0;

        nums.sort_unstable();
        nums.push(i32::MAX);

        for i in 0..nums.len() - 1 {
            let num: i32 = nums[i];

            let left: usize = nums.partition_point(|x| x < &(lower - num)).max(i + 1);
            let right: usize = nums.partition_point(|x| x <= &(upper - num)).max(i + 1);

            res += (right - left) as i64;
        }

        res
    }
}
/*
Solution: Prefix Sum

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 4.06 MB | 50.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut prefix: Vec<i64> = Vec::new();
        let mut acc: i64 = 0;

        for &num in nums.iter() {
            acc += num as i64;
            prefix.push(acc);
        }

        let mut res: i32 = 0;

        for i in 0..n - 1 {
            if prefix[n - 1] - prefix[i] <= prefix[i] {
                res += 1;
            }
        }

        res
    }
}
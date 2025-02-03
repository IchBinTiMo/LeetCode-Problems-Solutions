/*
Solution: 

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.28 MB | 50.00%

- n: length of nums
*/

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut inc: Vec<i32> = Vec::new();
        let mut dec: Vec<i32> = Vec::new();

        let mut res: usize = 0;

        for &num in nums.iter() {
            while !inc.is_empty() && *inc.last().unwrap() >= num {
                inc.clear();
            }

            inc.push(num);

            while !dec.is_empty() && *dec.last().unwrap() <= num {
                dec.clear();
            }

            dec.push(num);

            res = res.max(dec.len().max(inc.len()));

        }

        res as i32
    }
}
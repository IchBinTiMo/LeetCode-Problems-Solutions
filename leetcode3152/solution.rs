/*
Solution: Prefix Sum

Time: O(n * q) | Space: O(n)

Runtime: 3 ms | 100.00%
Memory: 10.02 MB | 75.00%

- n: length of 'nums'
- q: length of 'queries'
*/

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut prefix: Vec<i32> = vec![1];
        let mut acc: i32 = 1;

        for i in 1..nums.len() {
            let valid: bool = (nums[i] ^ nums[i - 1]) & 1 == 1;

            acc += valid as i32;

            prefix.push(acc);
        }

        let mut res: Vec<bool> = Vec::new();

        for q in queries {
            let from: usize = q[0] as usize;
            let to: usize = q[1] as usize;

            if from == to {
                res.push(true);
            } else {
                res.push((from - to) == (prefix[from] - prefix[to]) as usize);
            }
        }

        res
    }
}
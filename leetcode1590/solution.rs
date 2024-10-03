/*
Solution: Prefix Sum

Time: O(n) | Space: O(n)

Runtime: 22 ms | 100.00%
Memory: 6.93 MB | 33.33%

- n: length of 'nums'
*/

use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {

        let n: usize = nums.len();

        let mut prefix: Vec<i32> = vec![0; n + 1];

        let rem: i32 = (nums.iter().map(|&v| v as i64).sum::<i64>() % p as i64) as i32;

        // to store the indices last seen of the cumulative remainders
        let mut shown: HashMap<i32, usize> = HashMap::new(); 

        shown.insert(0, 0);

        let mut res: usize = usize::MAX;

        for i in 1..=n {
            // calculate the prefix remainder x,
            // update the shown map,
            // and then find a shown remainder y such that (x + p * a) - y = rem, where a is greater than 0
            prefix[i] = prefix[i - 1] + (nums[i - 1] % p);

            shown.insert((prefix[i] % p), i);

            prefix[i] %= p;

            if let Some(idx) = shown.get(&((prefix[i] - rem + p) % p)) {
                if i - idx != n {
                    // we cant delete the entire array
                    res = res.min(i - idx);
                }
            }

        }

        return if res == usize::MAX {
            -1
        } else {
            res as i32
        }
    }
}
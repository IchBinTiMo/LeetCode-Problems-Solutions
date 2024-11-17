/*
Solution: Sliding Window + Monotonic Queue

Time: O(n) | Space: O(n)

Runtime: 6 ms | 75.00%
Memory: 3.42 MB | 100.00%

- n: length of 'nums'
*/

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let k: i64 = k as i64;
        
        let mut res: usize = usize::MAX;
        let mut acc: i64 = 0;
        let mut q: VecDeque<(i64, usize)> = VecDeque::new();

        for right in 0..nums.len() {
            acc += nums[right] as i64;

            if acc >= k {
                res = res.min(right + 1);
            }

            while !q.is_empty() && acc - q.front().unwrap().0 >= k {
                let (_, left) = q.pop_front().unwrap();

                res = res.min(right - left);
            }

            while !q.is_empty() && q.back().unwrap().0 > acc {
                q.pop_back();
            }

            q.push_back((acc, right));
        }        

        if res == usize::MAX {
            -1
        } else {
            res as i32
        }
    }
}
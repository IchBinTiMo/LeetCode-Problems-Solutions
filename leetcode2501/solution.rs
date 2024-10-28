/*
Solution 1: DP

Time: O(n log n) | Space: O(n)

Runtime: 10 ms | 100.00%
Memory: 3.7 MB | 100.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut res: i32 = -1;

        let mut dp: Vec<i32> = vec![-1; 100001];
        let mut prev: i32 = -1;

        for &num in nums.iter() {
            if prev == num {
                continue;
            } else {
                prev = num;
            }

            let num: usize = num as usize;

            if num * num <= 100001 {
                dp[num * num] = num as i32;
            }

            if dp[num] > 1 {
                dp[num] = dp[dp[num] as usize] + 1;

                res = res.max(dp[num]);
            } else {
                dp[num] = 1;
            }
        }

        res
    }
}

/*
Solution 2: HashMap

Time: O(n log n) | Space: O(n)

Runtime: 43 ms | 100.00%
Memory: 6.44 MB | 100.00%

- n: length of 'nums'
*/

use std::collections::HashMap;

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut prev: HashMap<i32, i32> = HashMap::new();
        let mut length: HashMap<i32, i32> = HashMap::new();

        let mut res: i32 = -1;

        for &num in nums.iter() {
            prev.insert(num * num, num);

            if let Some(p) = prev.get(&num) {
                let tmp: i32 = *length.get(&p).unwrap();
                length.insert(num, tmp + 1);
                res = res.max(tmp + 1);
            } else {
                length.insert(num, 1);
            }
        }

        res
    }
}
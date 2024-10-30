/*
Solution2: DP + Brute Force

Time: O(n ^ 2) | Space: O(n)

Runtime: 27 ms | 100.00%
Memory: 2.26 MB | 100.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        let mut forward: Vec<i32> = vec![0; n];
        let mut backward: Vec<i32> = vec![0; n];

        for i in 1..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    forward[i] = forward[i].max(forward[j] + 1);
                }
            }
        }


        for i in (0..(n - 1)).rev() {
            for j in (i + 1)..n {
                if nums[i] > nums[j] {
                    backward[i] = backward[i].max(backward[j] + 1);
                }
            }
        }

        let mut res: i32 = n as i32;

        for i in 0..n {
            if forward[i] * backward[i] == 0 {
                continue;
            } else {
                res = res.min((n as i32) - (forward[i] + backward[i] + 1));
            }
        }


        res
    }
}
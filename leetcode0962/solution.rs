/*
Solution 1: Monotonic Stack

Time: O(n) | Space: O(n)

Runtime: 7 ms | 20.00%
Memory: 2.93 MB | 20.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        let mut res: usize = usize::MIN;

        let mut dec: Vec<usize> = Vec::new();

        for i in 0..n {
            if dec.is_empty() || nums[*dec.last().unwrap()] > nums[i] {
                dec.push(i);
            }
        }

        for i in (0..n).rev() {
            while let Some(&left) = dec.last() {
                if nums[left] <= nums[i] {
                    res = res.max(i - left);
                    dec.pop();
                } else {
                    break;
                }
            }
        }

        res as i32
    }
}

/*
Solution 2:

Time: O(n) | Space: O(1)

Runtime: 5 ms | 60.00%
Memory: 2.74 MB | 20.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        let mut res: usize = usize::MIN;

        let mut first: Vec<usize> = vec![usize::MAX; 50001];

        for i in 0..n {
            let current: usize = nums[i] as usize;

            if first[current] == usize::MAX {
                first[current] = i;
            }
        }

        for i in 1..50001 {
            first[i] = first[i].min(first[i - 1]);
        }

        for i in (0..n).rev() {
            let current: usize = nums[i] as usize;
            if i >= first[current] {
                res = res.max(i - first[current]);
            }
        }

        res as i32

    }
}
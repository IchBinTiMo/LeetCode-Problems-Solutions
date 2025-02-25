/*
Solution: DP, Prefix Sum

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 3.20 MB | 60.00
*/

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let n: usize = arr.len();

        let mut prev: (i32, i32) = (0, 0);

        let mut even: i32 = 0;
        let mut odd: i32 = 0;

        for i in 0..n {
            let num: i32 = arr[i];

            if num & 1 == 1 {
                prev = (prev.1 + 1, prev.0);
            } else {
                prev = (prev.0, prev.1 + 1);
            }

            odd = (odd + prev.0) % 1_000_000_007;
            even = (even + prev.1) % 1_000_000_007;
        }

        odd
    }
}
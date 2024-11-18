/*
Solution: Sliding Window

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.18 MB | 61.11%

- n: length of 'code'
*/

impl Solution {
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let n: usize = code.len();
        let mut res: Vec<i32> = vec![0; n];

        let mut left: usize = 1;
        let mut right: usize = 1;

        if k < 0 {
            left = n + k as usize;
            right = n + k as usize;
        } else if k == 0 {
            return res;
        }

        let mut sum: i32 = 0;
        let mut cnt: i32 = 0;

        while cnt < k.abs() {
            sum += code[right];
            right = (right + 1) % n;
            cnt += 1;
        }

        for i in 0..n {
            res[i] = sum;

            sum -= code[left];
            left = (left + 1) % n;

            sum += code[right];
            right = (right + 1) % n;
        }

        res
    }
}
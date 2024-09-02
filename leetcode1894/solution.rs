/*
Solution: 

Time: O(n) | Space: O(1)

Runtime: 12 ms | 76.00%
Memory: 3.26 MB | 52.00%

- n: length of 'chalk'
*/

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let n: usize = chalk.len();
        let mut current: usize = 0;
        let mut k: i64 = k as i64;

        let mut sum: i64 = 0;

        for &c in chalk.iter() {
            sum += c as i64;
        }

        k = if k >= sum {
            k % sum
        } else {
            k
        };

        let mut k: i32 = k as i32;

        while chalk[current % n] <= k {
            k -= chalk[current % n];
            current += 1
        }

        (current % n) as i32
    }
}
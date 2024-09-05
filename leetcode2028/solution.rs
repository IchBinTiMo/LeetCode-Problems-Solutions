/*
Solution: 

Time: O(N) | Space: O(n)

Runtime: 51 ms | 87.50%
Memory: 2.90 MB | 100.00%

- N: length of 'rolls' + 'n'

*/

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, mut n: i32) -> Vec<i32> {
        let m: i32 = rolls.len() as i32;

        let mut remain: i32 = (m + n) * mean - rolls.iter().sum::<i32>();

        if remain > 6 * n || remain < n {
            return Vec::new();
        }

        let mut res: Vec<i32> = vec![remain / n; n as usize];

        remain %= n;

        for i in 0..remain as usize {
            res[i] += 1;
        }

        res
    }
}
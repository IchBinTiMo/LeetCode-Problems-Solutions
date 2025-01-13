/*
Solution: Counting

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.84 MB | 100.00%

- n: length of 's'
*/

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut res: usize = s.len();

        let mut freqs: [usize; 26] = [0; 26];

        for b in s.bytes() {
            freqs[(b - b'a') as usize] += 1;
        }

        for i in 0..26 {
            if freqs[i] == 0 {
                continue;
            }

            res -= freqs[i] - (freqs[i] & 1 ^ 1) - 1; 
        }

        res as i32
    }
}
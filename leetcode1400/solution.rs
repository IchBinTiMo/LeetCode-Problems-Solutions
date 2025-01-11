/*
Solution: HashMap

Time: O(n) | Space: O(1)

Runtim: 0 ms | 100.00%
Memory: 2.50 MB | 100.00%

- n: length of 's'
*/

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut freqs: [i32; 26] = [0; 26];

        for b in s.bytes() {
            freqs[(b - b'a') as usize] += 1;
        }

        let mut odd: i32 = 0;
        let mut even: i32 = 0;

        for i in 0..26 {
            odd += freqs[i] & 1;
            even += freqs[i] - (freqs[i] & 1);

            if odd > k {
                return false;
            }
        }

        odd <= k && odd + even >= k
    }
}
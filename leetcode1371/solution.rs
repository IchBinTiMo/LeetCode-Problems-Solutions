/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 3 ms | 100.00%
Memory: 3.15 MB | 61.54%

- n: length of 's'
*/

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        // the frequency of each vowel
        // we only use the rightmost 5 bits to track the frequency
        // if a bit is 0, the frequency is even
        // otherwise, the frequency is odd
        let mut freqs: usize = 0;

        // the first index of such frequency occurs
        let mut appear: [i32; 32] = [i32::MAX; 32];

        // initialize
        appear[0] = -1;

        let mut res: i32 = 0;

        for (i, c) in s.chars().enumerate() {
            let i: i32 = i as i32;

            // update the frequency
            match c {
                'a' => freqs ^= (1 << 4),
                'e' => freqs ^= (1 << 3),
                'i' => freqs ^= (1 << 2),
                'o' => freqs ^= (1 << 1),
                'u' => freqs ^= 1,
                _ => {}
            }

            if appear[freqs] == i32::MAX {
                // if the frequency is not found, set it
                appear[freqs] = i;
            } else {
                // if the frequency is found, update the result
                res = res.max(i - appear[freqs]);
            }
        }

        res
    }
}
/*
Solution: 

Time: O(n) | Space: O(1)

Runtime: 5 ms | 100.00%
Storage: 2.52 MB | 50.00%

- n: length of 's'
*/

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let mut left: [i32; 26] = [-1; 26];
        let mut right: [i32; 26] = [-1; 26];

        for (i, b) in s.bytes().enumerate() {
            let b: usize = (b - b'a') as usize;

            if left[b] == -1 {
                left[b] = i as i32;
            }

            right[b] = i as i32;
        }

        let mut res: i32 = 0;

        let s = s.as_bytes();

        for i in 0..26 {
            let mut visited: [bool; 26] = [false; 26];

            if right[i] == -1 {
                continue;
            }

            for j in (left[i] + 1)..right[i] {
                let b: usize = (s[j as usize] - b'a') as usize;

                if !visited[b] {
                    visited[b] = true;
                    res += 1;
                }
            }
        }

        res
    }
}
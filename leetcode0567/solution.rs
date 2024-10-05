/*
Solution: Two Pointers

Time: O(m + n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 9.52%

- m: length of 's1'
- n: length of 's2'
*/

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s2: Vec<char> = s2.chars().collect::<Vec<char>>();

        let mut freqs_1: [i32; 26] = [0; 26];
        let mut freqs_2: [i32; 26] = [0; 26];

        for c in s1.chars() {
            freqs_1[(c as usize) - ('a' as usize)] += 1;
        }

        let mut left: usize = 0;
        let mut right: usize = 0;

        while right < s2.len() {
            if right - left < s1.len() {
                freqs_2[(s2[right] as usize) - ('a' as usize)] += 1;
                right += 1;
            } else if right - left == s1.len() {
                if freqs_1 == freqs_2 {
                    return true;
                } else {
                    freqs_2[(s2[left] as usize) - ('a' as usize)] -= 1;
                    left += 1;
                }
            }
        }
        freqs_1 == freqs_2

    }
}
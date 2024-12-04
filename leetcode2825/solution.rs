/*
Solution: Two Pointers

TIme: O(m + n) | Space: O(m + n)

Runtime: 4 ms | 100.00%
Memory: 2.64 MB | 100.00%

- m: length of 'str1'
- n: length of 'str2'
*/

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let str1: Vec<u8> = str1.bytes().collect::<Vec<u8>>();
        let str2: Vec<u8> = str2.bytes().collect::<Vec<u8>>();

        let mut left: usize = str1.len() - 1;
        let mut right: usize = str2.len() - 1;

        while left < str1.len() && right < str2.len() {
            if str1[left] == str2[right] || (str1[left] + 1 - b'a') % 26 == str2[right] - b'a' {
                right -= 1;
            }

            left -= 1;
        }

        right == usize::MAX
    }
}
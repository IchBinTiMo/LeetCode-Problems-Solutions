/*
Solution: Two Pointers

Time: O(m + n) | Space: O(m + n)

Runtime: 1 ms | 66.67%
Memory: 2.13 MB | 66.67%

- m: length of 'sentence1'
- n: length of 'sentence2'
*/

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        if sentence1 == sentence2 {
            return true;
        }

        let s1: Vec<&str> = sentence1.split(' ').collect::<Vec<&str>>();
        let s2: Vec<&str> = sentence2.split(' ').collect::<Vec<&str>>();

        let m: usize = s1.len();
        let n: usize = s2.len();

        let mut left1: usize = 0;
        let mut right1: usize = m - 1;

        let mut left2: usize = 0;
        let mut right2: usize = n - 1;

        let mut flag: u8 = 0;

        if m <= n {
            while left1 <= m - 1 {
                if s1[left1] == s2[left2] {
                    left1 += 1;
                    left2 += 1
                } else {
                    break;
                }
            }

            while right1 >= left1 && right1 < m {
                if s1[right1] == s2[right2] {
                    right1 -= 1;
                    right2 -= 1;
                } else {
                    break;
                }
            }

            return right1 >= m || (right1 as i32) - (left1 as i32) < 0;
        } else {
            while left2 <= n - 1 {
                if s1[left1] == s2[left2] {
                    left1 += 1;
                    left2 += 1
                } else {
                    break;
                }
            }

            while right2 >= left2 && right2 < n {
                if s1[right1] == s2[right2] {
                    right1 -= 1;
                    right2 -= 1;
                } else {
                    break;
                }
            }

            return right2 >= n || (right2 as i32) - (left2 as i32) < 0;
        }
    }
}
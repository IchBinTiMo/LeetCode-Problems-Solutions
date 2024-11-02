/*
Solution:

Runtime: 0 ms | 100.00%
Memory: 2.14 MB | 50.00%

Time: O(n) | Space: O(1)

- n: length of 'sentence'
*/

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        if sentence.chars().next().unwrap() != sentence.chars().last().unwrap() {
            return false;
        }

        let mut prev: char = '0';

        let strings = sentence.split(" ");

        for part in strings {
            let current = part.chars().next().unwrap();
            if prev == '0' {
                prev = part.chars().last().unwrap();
            } else {
                if prev != current {
                    return false;
                } else {
                    prev = part.chars().last().unwrap();
                }
            }
        }

        true
    }
}
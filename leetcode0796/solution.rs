/*
Solution: 

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.16 MB | 26.47%

- n: length of 's'
*/

use std::collections::VecDeque;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let mut s: VecDeque<char> = s.chars().collect::<VecDeque<char>>();
        let goal: VecDeque<char> = goal.chars().collect::<VecDeque<char>>();

        let n: usize = s.len();

        let mut i: usize = 0;

        while i < n {
            if s == goal {
                return true;
            } else {
                let c: char = s.pop_front().unwrap();

                s.push_back(c);
            }

            i += 1;
        }

        false
    }
}
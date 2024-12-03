/*
Solution: 

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 6.24 MB | 89.47%

- n: length of 's'
*/

impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut res: String = String::new();

        let mut idx: usize = 0;

        for (i, c) in s.chars().enumerate() {
            if idx < spaces.len() && i == spaces[idx] as usize {
                res.push(' ');
                idx += 1;
            }

            res.push(c);
        }

        res
    }
}
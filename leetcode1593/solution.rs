/*
Solution: Backtracking

Time: O(2^n * n) | Space: O(n)

Runtime: 89 ms | 100.00%
Memory: 2.14 MB | 100.00%

- n: length of 's'
*/

use std::collections::HashSet;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut set: HashSet<String> = HashSet::new();

        let mut current: String = String::new();

        let mut res: usize = usize::MIN;

        Self::backtrack(&mut res, &mut set, &mut current, &s[0..s.len()]);

        res as i32
    }

    fn backtrack(res: &mut usize, path: &mut HashSet<String>, current: &mut String, s: &str) {
        if path.len() > *res {
            *res = path.len();
        }

        for (i, c) in s.chars().enumerate() {
            *current = format!("{current}{c}");

            if let Some(_) = path.get(current) {
                continue;
            } else {
                path.insert(format!("{current}"));

                let mut tmp: String = String::new();
                Self::backtrack(res, path, &mut tmp, &s[(i + 1)..s.len()]);
                
                path.remove(&format!("{current}"));
            }
        }

        
    }
}
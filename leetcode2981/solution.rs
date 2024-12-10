/*
Solution: HashMap

Time: O(n ^ 3) | Space: O(n ^ 2)

Runtime: 0 ms | 100.00%
Memory: 2.33 MB | 100.00%

- n: length of 's'
*/

use std::collections::HashMap;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut map: HashMap<&str, i32> = HashMap::new();

        for i in 0..s.len() {
            for j in i..s.len() {
                if &s[i..=i] != &s[j..=j] {
                    break;
                }
                map.entry(&s[i..=j]).and_modify(|cnt| *cnt += 1).or_insert(1);
            }
        }

        let mut res: i32 = -1;

        let mut current: usize = 0;

        for (k, &v) in map.iter() {

            if v >= 3 && k.len() >= current {
                res = k.len() as i32;
                current = k.len();
            }
        }

        res
    }
}
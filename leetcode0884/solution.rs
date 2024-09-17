/*
Solution:

Time: O(m + n) | Space: O(m + n)

Runtime: 0 ms | 100.00%
Memory: 2.14 MB | 62.50%

- m: length of 's1'
- n: length of 's2'
*/

use std::collections::HashMap;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let s1: Vec<String> = s1.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let s2: Vec<String> = s2.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();

        let mut map1: HashMap<String, i32> = HashMap::new();
        let mut map2: HashMap<String, i32> = HashMap::new();

        for s in s1 {
            map1.entry(s).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        for s in s2 {
            if map1.get(&s).is_none() {
                map2.entry(s).and_modify(|cnt| *cnt += 1).or_insert(1);
            } else {
                map1.entry(s).and_modify(|cnt| *cnt += 1);
            }
        }

        let mut res: Vec<String> = Vec::new();

        for key in map1.keys() {
            if let Some(cnt) = map1.get(key) {
                if *cnt == 1 {
                    res.push(key.to_string());
                }
            }
        }

        for key in map2.keys() {
            if let Some(cnt) = map2.get(key) {
                if *cnt == 1 {
                    res.push(key.to_string());
                }
            }
        }

        res
    }
}
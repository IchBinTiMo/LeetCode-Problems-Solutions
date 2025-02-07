/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 22 ms | 100.00%
Memory: 11.73 MB | 100.00%

- n: length of 'queries'
*/

use std::collections::HashMap;

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut current: HashMap<i32, i32> = HashMap::new();
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut res: Vec<i32> = Vec::new();

        for q in queries {
            let ball: i32 = q[0];
            let color: i32 = q[1];

            if let Some(&clr) = current.get(&ball) {
                map.entry(clr).and_modify(|cnt| *cnt -= 1);

                if let Some(&cnt) = map.get(&clr) {
                    if cnt == 0 {
                        map.remove(&clr);
                    }
                }
            }

            current.insert(ball, color);

            map.entry(color).and_modify(|cnt| *cnt += 1).or_insert(1);

            res.push(map.keys().len() as i32);
        }

        res
    }
}
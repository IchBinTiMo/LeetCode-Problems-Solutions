/*
Solution: 

Time: O(n) | Space: O(n)

Runtime: 1 ms | 84.62%
Memory: 2.39 MB | 93.33%

- n: length of 'edges'
*/

use std::collections::HashSet;

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut set: HashSet<i32> = HashSet::from_iter(0..n);

        for edge in edges {
            let v: i32 = edge[1];

            if set.contains(&v) {
                set.remove(&v);
            }
        }

        let mut res: i32 = -1;

        if set.len() == 1 {
            for &tmp in set.iter() {
                res = tmp;
            }
        }
        
        res
    }
}
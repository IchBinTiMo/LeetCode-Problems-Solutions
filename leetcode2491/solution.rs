/*
Solution: 

Time: O(n) | Space: O(n)

Runtime: 4 ms | 100.00%
Memory: 2.91 MB | 66.67%

- n: length of 'skill'
*/

use std::collections::HashMap;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let n: i32 = skill.len() as i32;

        let sum: i32 = skill.iter().sum::<i32>();

        if 2 * sum % n != 0 {
            return -1
        }

        let target: i32 = 2 * sum / n;

        let mut freqs: HashMap<i32, i32> = HashMap::new();

        let mut res: i64 = 0;

        for &s in skill.iter() {
            if let Some(cnt) = freqs.get_mut(&(target - s)) {
                *cnt -= 1;
                if *cnt == 0 {
                    freqs.remove(&(target - s));
                }
                res += (s as i64) * (target - s) as i64;
            } else {
                freqs.entry(s).and_modify(|cnt| *cnt += 1).or_insert(1);
            }
        }

        if freqs.len() > 0 {
            return -1
        }

        res
    }
}
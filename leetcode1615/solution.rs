use std::collections::HashMap;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut ans: usize = 0;
        let mut map: HashMap<u8, Vec<u8>> = HashMap::new();

        for road in roads {
            let a = road[0] as u8;
            let b = road[1] as u8;
            map.entry(a).or_insert(Vec::new()).push(b);
            map.entry(b).or_insert(Vec::new()).push(a);
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let a = map.get(&(i as u8)).unwrap_or(&vec![]).len();
                let b = map.get(&(j as u8)).unwrap_or(&vec![]).len();
                let mut tmp = a + b;
                if map.get(&(i as u8)).unwrap_or(&vec![]).contains(&(j as u8)) {
                    tmp -= 1;
                }

                ans = ans.max(tmp);
            }
        }

        ans as i32
    }
}
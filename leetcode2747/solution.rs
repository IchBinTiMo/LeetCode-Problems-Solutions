use std::collections::HashMap;

impl Solution {
    pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let mut window: HashMap<i32, i32> = HashMap::new(); // <key, value>
        let mut ans: Vec<i32> = vec![0; queries.len()];

        let mut qp: Vec<usize> = (0..queries.len()).collect();

        qp.sort_unstable_by_key(|&i| queries[i]);

        logs.sort_unstable_by_key(|l| l[1]);

        let mut left = 0;
        let mut right = 0;

        for pos in qp {
            while right < logs.len() && logs[right][1] <= queries[pos] {
                *window.entry(logs[right][0]).or_insert(0) += 1;
                right += 1;
            }

            while left < logs.len() && logs[left][1] < queries[pos] - x {
                *window.entry(logs[left][0]).or_insert(0) -= 1;
                if *window.get(&logs[left][0]).unwrap() <= 0 {
                    window.remove(&logs[left][0]);
                }
                left += 1;
            }

            ans[pos] = n - window.len() as i32
        }

        ans
    }
}
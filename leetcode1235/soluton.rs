impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<usize> = (0..profit.len()).collect();
        jobs.sort_unstable_by_key(|&i| end_time[i]);

        let mut dp: Vec<(i32, i32)> = vec![(0, i32::MIN)];

        for &i in jobs.iter() {
            let left = match dp.binary_search_by_key(&start_time[i], |&(_, time)| time) {
                Ok(j) => j,
                Err(j) => j - 1
            };

            if let Some(pair) = dp.last() {
                if dp[left].0 + profit[i] > pair.0 {
                    if end_time[i] == pair.1 {
                        dp.last_mut().unwrap().0 = dp[left].0 + profit[i];
                    } else {
                        dp.push((dp[left].0 + profit[i], end_time[i]));
                    }
                }
            }
        }

        dp.last().unwrap().0
       
    }
}
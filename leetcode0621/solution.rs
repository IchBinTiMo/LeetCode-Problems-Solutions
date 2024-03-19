impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        // count the occurrence of each task
        let mut freq: [i32; 26] = [0; 26];

        // find the max frequency
        let mut max_freq: i32 = 0;

        // find th count of tasks with the max frequency
        let mut n_max_freq: i32 = 0;

        for &task in tasks.iter() {
            freq[task as usize - 'A' as usize] += 1;

            if max_freq == freq[task as usize - 'A' as usize] {
                n_max_freq += 1;
            } else if max_freq < freq[task as usize - 'A' as usize] {
                max_freq = freq[task as usize - 'A' as usize];
                n_max_freq = 1;
            }
        }


        let gap_count: i32 = max_freq - 1;

        // the length remained in each gap
        let length_remained: i32 = n - (n_max_freq - 1);

        // find currently needed count of task
        let gaps: i32 = gap_count * length_remained;

        // find the remaining task
        let task_remained: i32 = tasks.len() as i32 - max_freq * n_max_freq;

        // find the result
        let res = 0.max(gaps - task_remained);

        res + tasks.len() as i32
    }
}
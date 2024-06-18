impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        /// Time: O(n log n) | Space: O(n)
        /// where n is the length of difficulty
        let n: usize = difficulty.len();

        let mut res: i32 = 0;

        // combined difficulty and profit into (difficulty, profit)
        let mut jobs: Vec<(i32, i32)> = (0..n).map(|i| (difficulty[i], profit[i])).collect::<Vec<(i32, i32)>>();

        let mut workers = worker;

        // sort jobs by profit
        jobs.sort_unstable_by_key(|job| job.1);

        // sort workers by their ability
        workers.sort_unstable();

        while let Some(worker) = workers.pop() {
            while let Some(&(diff, prof)) = jobs.last() {
                if diff > worker {
                    // if the job difficulty is greater than current worker's ability,
                    // there is no remained worker can do this job,
                    // so we pop the job
                    jobs.pop();
                } else {
                    // a worker can only do exactly one job
                    // so we break the loop
                    // whenever the job difficulty is less than current worker's ability
                    res += prof;
                    break;
                }
            }
        }

        res
    }
}
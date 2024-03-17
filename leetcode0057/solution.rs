impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let n: usize = intervals.len();

        let mut res: Vec<Vec<i32>> = Vec::new();

        let mut idx: usize = 0;

        // add all the intervals that end before the new interval starts
        while idx < n && intervals[idx][1] < new_interval[0] {
            res.push(intervals[idx].clone());
            idx += 1;
        }

        // combine intervals until there is no overlap
        while idx < n && intervals[idx][1] >= new_interval[0] && new_interval[1] >= intervals[idx][0] {
            new_interval[0] = new_interval[0].min(intervals[idx][0]);
            new_interval[1] = new_interval[1].max(intervals[idx][1]);
            idx += 1;
        }

        res.push(new_interval);

        // add all the remaining intervals
        while idx < n {
            res.push(intervals[idx].clone());
            idx += 1;
        }

        res
    }
}
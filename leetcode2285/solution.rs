impl Solution {
    pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
        /// Time O(NlogN) | Space O(N)
        /// where N is the length of roads
        let n: usize = n as usize;

        let mut freqs: Vec<i64> = vec![0; n];

        for road in roads {
            freqs[road[0] as usize] += 1;
            freqs[road[1] as usize] += 1;
        }

        freqs.sort_unstable();

        let mut res: i64 = 0;

        for i in 0..n {
            res += freqs[i] * (i + 1) as i64;
        }

        res
    }
}
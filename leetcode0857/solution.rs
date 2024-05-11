use std::collections::BinaryHeap;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        /// https://leetcode.com/problems/minimum-cost-to-hire-k-workers/solutions/5141607/rust-3ms-solution-with-explain
        /// Time: O(n log n) | Space: O(n)
        let n: usize = quality.len();
        let k: usize = k as usize;

        // sort workers by their ratio of their wage to their quality
        let mut workers: Vec<(f64, i32)> = quality.iter().zip(wage.iter()).map(|(&q, &w)| (w as f64 / q as f64, q)).collect();
        workers.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut quality_sum: i32 = 0;

        let mut heap: BinaryHeap<i32> = BinaryHeap::new();

        // the answer is the sum of the top k smallest quality workers times the maximum wage to quality ratio of these workers
        // since the wage to quality ratio is in ascending order
        // we can use a heap to handle the k smallest quality workers
        // and multiply it by the wage to quality ratio of the latest worker comes in 
        workers.iter().fold(f64::MAX, |res, &(wq_ratio, q)| {
            heap.push(q);
            quality_sum += q;

            if heap.len() > k {
                quality_sum -= heap.pop().unwrap();
            }

            if heap.len() == k {
                res.min(quality_sum as f64 * wq_ratio)
            } else {
                res
            }
        }) as f64
    }
}
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_maximized_capital(mut k: i32, mut w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        /// Time: O(n log n + k log n) | Space: O(n + k)
        /// where n is the length of profits
        let n: usize = profits.len();
        let mut heap: BinaryHeap<i32> = BinaryHeap::new(); // max heap

        let mut tasks: Vec<(i32, i32)> = (0..n).map(|idx| (profits[idx], capital[idx])).collect::<Vec<(i32, i32)>>();

        // sort tasks by capital they need
        tasks.sort_unstable_by_key(|t| t.1);

        let mut res: i32 = w;

        let mut idx: usize = 0;

        while k > 0 {
            // add all the tasks that can be done with the current amount of capital
            while idx < n && tasks[idx].1 <= w {
                heap.push(tasks[idx].0);
                idx += 1;
            }

            // pop the profit with the highest profit
            if let Some(profit) = heap.pop() {
                res += profit;
                w += profit;
            }

            k -= 1;
        }

        res
    }
}
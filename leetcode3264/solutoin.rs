/*
Solution: Binary Heap

Time: O(n log n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.40 MB | 20.00%

- n: length of 'nums'
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        let n: usize = nums.len();

        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();

        for i in 0..nums.len() {
            heap.push((-nums[i], -(i as i32)));
        }

        for _ in 0..k {
            if let Some((num, idx)) = heap.pop() {
                heap.push((multiplier * num, idx));
            }
        }

        let mut res: Vec<i32> = vec![0; n];

        while let Some((num, idx)) = heap.pop() {
            let num: i32 = -num;
            let idx: usize = -idx as usize;

            res[idx] = num;
        }

        res
    }
}
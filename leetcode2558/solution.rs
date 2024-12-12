/*
Solution: Binary Heap

Time: O(n * k) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.68 MB | 12.50%

- n: length of 'gifts'
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
        let mut heap: BinaryHeap<i64> = BinaryHeap::new();
        let mut res: i64 = 0;

        for &g in gifts.iter() {
            heap.push(g as i64);
        }

        for _ in 0..k {
            if let Some(g) = heap.pop() {
                let g: f64 = g as f64;
                let new: f64 = g.sqrt().floor();

                heap.push(new as i64);
            }
        }

        while let Some(g) = heap.pop() {
            res += g;
        }

        res
    }
}
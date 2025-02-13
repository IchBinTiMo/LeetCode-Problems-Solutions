/*
Solution: Heap

Time: O(nlogn) | Space: O(n)

Runtime: 29 ms | 76.00%
Runtime: 295.36 ms | 76.00%

- n: length of 'nums'
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = nums
            .iter()
            .map(|&n| -n as i64)
            .collect::<BinaryHeap<_>>();

        let mut res: i32 = 0;

        loop {
            let first: Option<i64> = heap.pop();

            if -first.unwrap() >= k as i64 {
                break;
            }

            let second: Option<i64> = heap.pop();

            if second.is_none() {
                break;
            }

            res += 1;
            heap.push(first.unwrap() * 2 + second.unwrap());

        }

        res
    }
}
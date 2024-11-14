/*
Solution 1: Binary Search

Time: O(q log M) | Space: O(1)

Runtime: 20 ms | 100.00%
Memory: 3.24 MB | 100.00%

- q: length of 'quantities'
- M: max of 'quantities'
*/

impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut low: i32 = 1;
        let mut high: i32 = *quantities.iter().max().unwrap();

        while low < high {
            let mid: i32 = (low + high) / 2;
            let mut cnt: i32 = 0;

            for &q in quantities.iter() {
                cnt += (q + mid - 1) / mid;
            }

            if cnt <= n {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low
    }
}

/*
Solution 2: Binary Heap

Time: O(q log q) | Space: O(q)

Runtime: 644 ms | 100.00%
Memory: 4.59 MB | 100.00%

- n: length of 'quantities'
- q: length of 'quantities'
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn minimized_maximum(mut n: i32, quantities: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<(i32, i32, usize)> = BinaryHeap::new();

        for i in 0..quantities.len() {
            heap.push((quantities[i], -1, i));
            n -= 1;
        }

        while n > 0 {
            let (maxi, cnt, i): (i32, i32, usize) = heap.pop().unwrap();

            let mut new_maxi: i32 = quantities[i] / (1 - cnt);

            if quantities[i] % (1 - cnt) != 0 {
                new_maxi += 1;
            }

            heap.push((new_maxi, cnt - 1, i));

            n -= 1;
        }

        heap.pop().unwrap().0
    }
}
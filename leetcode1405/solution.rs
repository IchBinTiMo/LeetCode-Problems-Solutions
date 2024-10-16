/*
Solution: heap

Time: O(n log n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.12 MB | 50.00%

- n: a + b + c 
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut heap: BinaryHeap<(i32, char)> = BinaryHeap::new();

        if a > 0 {
            heap.push((a, 'a'));
        }

        if b > 0 {
            heap.push((b, 'b'));
        }

        if c > 0 {
            heap.push((c, 'c'));
        }

        let mut res: String = String::new();

        let mut prev: char = 'z';

        while let Some((mut cnt, c)) = heap.pop() {
            if c == prev {
                if let Some((mut next_cnt, mut next_c)) = heap.pop() {

                    prev = next_c;

                    let time: i32 = if cnt - next_cnt > 0 {1} else {2};
                    
                    for _ in 0..next_cnt.min(time) {
                        res = format!("{res}{next_c}");
                        next_cnt -= 1;
                    }

                    if next_cnt > 0 {
                        heap.push((next_cnt, next_c));
                    }

                    heap.push((cnt, c));
                } else {
                    break;
                }
            } else {
                prev = c;
                for _ in 0..2.min(cnt) {
                    res = format!("{res}{c}");
                    cnt -= 1;
                }

                if cnt > 0 {
                    heap.push((cnt, c));
                }
            }
        }

        res
    }
}
/*
Solution: heap

Time: O(n log(n)) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.07 MB | 83.33%

- n: # of digits of 'nums'
*/

use std::collections::BinaryHeap;

impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let mut heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut pos: usize = 0;

        while num > 0 {
            heap.push((num % 10, pos));
            num /= 10;
            pos += 1;
        }

        let mut digits: Vec<i32> = vec![0; pos];
        let mut swap: (usize, usize) = (usize::MAX, usize::MAX);
        let mut mx: i32 = i32::MIN;

        pos -= 1;

        while let Some((digit, p)) = heap.pop() {
            if p != pos  {
                if digit >= mx || swap.0 == usize::MAX {
                    swap.0 = p;
                    swap.1 = pos;
                    mx = digit;
                } else {
                    pos -= 1;
                }
            } else {
                pos -= 1;
            }

            digits[p] = digit;
        }

        if swap.0 != usize::MAX {
            digits[swap.0] ^= digits[swap.1];
            digits[swap.1] = digits[swap.0] ^ digits[swap.1];
            digits[swap.0] ^= digits[swap.1];
        }

        digits.iter().rev().fold(0, |acc, &n| acc * 10 + n)
    }
}
/*
Solution: Prefix Sum

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.35 MB | 30.00%

- n: length of 'boxes'
*/

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let n: usize = boxes.len();
        let boxes: &[u8] = boxes.as_bytes();

        let mut left: Vec<i32> = vec![0; n];
        let mut right: Vec<i32> = vec![0; n];

        let mut acc: i32 = 0;

        for i in 1..n {
            acc += if boxes[i - 1] == 49 {1} else {0};
            left[i] += left[i - 1] + acc;
        }

        acc = 0;

        for i in (0..n - 1).rev() {
            acc += if boxes[i + 1] == 49 {1} else {0};
            right[i] += right[i + 1] + acc;
        }

        (0..n).map(|i| left[i] + right[i]).collect::<Vec<i32>>() 
    }
}
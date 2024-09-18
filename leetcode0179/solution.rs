/*
Solution:

Time: O(n log(n)) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.27 MB | 58.33%

- n: length of 'nums'
*/

use std::cmp::Ordering;

impl Solution {
    pub fn largest_number(mut nums: Vec<i32>) -> String {
        nums.sort_unstable_by(|&a, &b| {
            format!("{a}{b}").cmp(&format!("{b}{a}"))
        });

        if *nums.last().unwrap() == 0 {
            return String::from("0");
        }

        let mut res: String = String::new();

        while let Some(n) = nums.pop() {
            res = format!("{res}{n}");
        }

        res
    }
}
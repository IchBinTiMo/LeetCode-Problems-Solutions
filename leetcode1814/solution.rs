use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut difference: HashMap<i32, i64> = HashMap::new();

        for num in nums.iter() {
            let mut rev = 0;
            let mut n = *num;

            while n > 0 {
                rev = rev * 10 + n % 10;
                n /= 10;
            }

            *difference.entry(*num - rev).or_insert(0) += 1;
        }

        difference.values().fold(0, |acc, &v| ((acc + v * (v - 1) / 2) % (1_000_000_007 as i64))) as i32
    }
}
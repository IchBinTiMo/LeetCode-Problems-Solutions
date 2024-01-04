use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut ht: HashMap<i32, i32> = HashMap::new();

        for &num in nums.iter() {
            *ht.entry(num).or_insert(0) += 1;
        }

        for &value in ht.values() {
            match value {
                1 => {
                    return -1;
                },
                _ => {
                    ans += ((value as f32) / (3 as f32)).ceil() as i32;
                }
            }
        }

        ans
    }
}
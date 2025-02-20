/*
Solution: 

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.34 MB | 14.29%

- n: length of 'nums'
*/

use std::collections::HashSet;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut visited: HashSet<i32> = HashSet::new();

        for num in nums.iter() {
            if let Ok(num) = i32::from_str_radix(num, 2) {
                visited.insert(num);
            }
        }

        for n in 0..(2_i32).pow(nums.len() as u32) {
            if !visited.contains(&n) {
                return format!("{:0len$b}", n, len=nums.len());
            }
        }

        String::new()
    }
}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut visited = 0;

        for num in nums.iter() {
            if let Ok(x) = i32::from_str_radix(num, 2) {
                visited |= 1 << x;
            }
        }

        for n in 0..(2_i32).pow(nums.len() as u32) {
            if visited & (1 << n) == 0{
                return format!("{:0len$b}", n, len=nums.len());
            }
        }
        String::new()
    }
}
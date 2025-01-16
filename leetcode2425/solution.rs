/*
Solution: 

Time: O(m + n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 4.32 MB | 40.00%

- m: length of 'nums1'
- n: length of 'nums2'
*/

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut num_1: i32 = 0;
        let mut num_2: i32 = 0;

        if nums2.len() & 1 == 1 {
            num_1 = nums1.iter().fold(0, |acc, &x| acc ^ x);
        }

        if nums1.len() & 1 == 1 {
            num_2 = nums2.iter().fold(0, |acc, &x| acc ^ x);
        }

        num_1 ^ num_2
    }
}
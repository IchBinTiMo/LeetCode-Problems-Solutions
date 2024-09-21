/*
Solution: Two Pointers

Time: O(n) | Space: O(1)

Runtime: 1 ms | 81.74%
Memory: 2.04 MB | 87.70%

- n: length of 'nums'
*/

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut n: usize = nums.len();

        let mut i: usize = 0;

        while i < n {
            if nums[i] == val {
                nums.swap_remove(i);
                n -= 1;
                i -= 1;
            }
            i += 1;
        }

        n as i32
    }
}
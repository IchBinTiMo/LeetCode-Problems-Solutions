/*
Solution: xor

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.76 MB | 100.00%

- n: length of 'derived'
*/

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.iter().fold(0, |acc, &x| acc ^ x) == 0
    }
}
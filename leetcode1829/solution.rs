/*
Solution: Bit Manipulation

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 3.53 MB | 100.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn get_maximum_xor(mut nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        let max: i32 = (1 << maximum_bit) - 1;

        let mut acc: i32 = nums.iter().fold(0, |acc, &num| acc ^ num);

        while let Some(num) = nums.pop() {
            res.push(max ^ acc);
            acc ^= num;
        }

        res
    }
}
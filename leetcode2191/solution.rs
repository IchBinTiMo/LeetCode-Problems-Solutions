/*
Solution 1:

Time: O(n log n) | Space: O(n)

- n: length of nums
*/

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut pairs: Vec<(i32, usize)> = Vec::new();

        for i in 0..nums.len() {
            let mut num: i32 = nums[i];
            let mut tmp: i32 = 0;
            let mut base: i32 = 1;

            if num != 0 {
                while num > 0 {
                    tmp += mapping[(num % 10) as usize] * base;
                    base *= 10;
                    num /= 10;
                }
            } else {
                tmp = mapping[0];
            }

            pairs.push((tmp, i));
        }

        pairs.sort_unstable();

        pairs.into_iter().map(|(n, i)| nums[i]).collect::<Vec<i32>>()
    }
}
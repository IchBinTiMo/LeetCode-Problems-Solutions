/*
Solution: 

Time: O(n^2) | Space: O(n^2)

Runtime: 57 ms | 100.00%
Memory: 11.26 MB | 50.00%

- n: length of 'nums'
*/

use std::collections::HashMap;

impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut res: i32 = 0;

        for i in 0..n {
            for j in (i + 1)..n {
                let prd: i32 = nums[i] * nums[j];

                map.entry(prd).and_modify(|cnt| {
                    res += *cnt * 8;
                    *cnt += 1;
                }).or_insert(1);
            }
        }

        res
    }
}
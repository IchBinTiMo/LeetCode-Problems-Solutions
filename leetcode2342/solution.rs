/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 15 ms | 17.65%
Memory: 4.14 MB | 20.59%

- n: length of 'nums'
*/

use std::collections::HashMap;

impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, [i32; 2]> = HashMap::new();

        let mut res: i32 = -1;

        for &num in nums.iter() {
            let mut n: i32 = num;
            let mut sum: i32 = 0;

            while n > 0 {
                sum += n % 10;
                n /= 10;
            }

            map
                .entry(sum)
                .and_modify(|arr| {
                    if num > arr[0] {
                        arr[1] = arr[0];
                        arr[0] = num;
                    } else if num > arr[1] {
                        arr[1] = num;
                    }
                    
                    res = res.max(arr[0] + arr[1]);
                })
                .or_insert([num, 0]);
        }

        res
    }
}
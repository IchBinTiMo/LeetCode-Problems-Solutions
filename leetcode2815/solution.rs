use std::collections::HashMap;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

        for num in nums {
            let mut n = num;
            let mut max = -1;
            while n > 0 {
                if n % 10 > max {
                    max = n % 10;
                }
                n /= 10;
            }
            (*map.entry(max).or_insert(Vec::new())).insert(0, num);
        }

        let mut max = -1;

        for (_k, mut v) in map {
            if v.len() < 2 {
                continue;
            }
            v.sort_by_key(|&x| -x );
            if v[0] + v[1] > max {
                max = v[0] + v[1];
            }
        }

        max
    }
}
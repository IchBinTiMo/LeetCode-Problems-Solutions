use std::collections::HashMap;

impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        /// Key: Count of Subarrays with K Distinct Elements = Count of Subarrays with K Distinct Elements - Count of Subarrays with K - 1 Distinct Elements
        let k: usize = k as usize;
        let n: usize = nums.len();

        // time each number appears
        let mut ht: HashMap<i32, i32> = HashMap::new();
        let mut ht_1: HashMap<i32, i32> = HashMap::new();

        // initialize
        let mut left: usize = 0;
        let mut left_1: usize = 0;

        let mut count: usize = 0; //Count of Subarrays with K Distinct Elements
        let mut count_1: usize = 0; //Count of Subarrays with K - 1 Distinct Elements

        // count
        for i in 0..n {
            ht.entry(nums[i])
                .and_modify(|e| *e += 1)
                .or_insert(1);

            while left <= i && ht.keys().len() > k {
                ht.entry(nums[left])
                    .and_modify(|e| *e -= 1);
                if let Some(&e) = ht.get(&nums[left]) {
                    if e == 0 {
                        ht.remove(&nums[left]);
                    }
                }
                left += 1;
            }

            count += i - left + 1;

                
            ht_1.entry(nums[i])
                .and_modify(|e| *e += 1)
                .or_insert(1);

             while left_1 <= i && ht_1.keys().len() > k - 1 {
                ht_1.entry(nums[left_1])
                    .and_modify(|e| *e -= 1);
                if let Some(&e) = ht_1.get(&nums[left_1]) {
                    if e == 0 {
                        ht_1.remove(&nums[left_1]);
                    }
                }
                left_1 += 1;
            }

            count_1 += i - left_1 + 1;
        }


        let res: usize = count - count_1;

        res as i32
    }
}
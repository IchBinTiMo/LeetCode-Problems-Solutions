use std::collections::HashSet;

impl Solution {
    pub fn can_traverse_all_pairs(mut nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        if nums.contains(&1) {
            return false;
        }

        let mut nums: Vec<i32> = HashSet::<i32>::from_iter(nums.iter().cloned()).into_iter().collect::<Vec<i32>>();
        
        let mut factor_pool: HashSet<i32> = Solution::get_factors(nums[0]);

        let mut connecting: bool = true;

        while connecting {
            connecting = false;

            let mut i: usize = 1;

            while i < nums.len() {

                let factor_list = Solution::get_factors(nums[i]);

                if !factor_pool.is_disjoint(&factor_list) {
                    connecting = true;
                    factor_pool.extend(factor_list);
                    nums.swap_remove(i);
                    continue;
                }

                i += 1;
            }
        }

        nums.len() == 1
    }

    fn get_factors(num: i32) -> HashSet<i32> {
        let mut res: HashSet<i32> = HashSet::new();
        for x in 1..=((num as f64).sqrt() as i32) {
            if num % x == 0 {
                res.insert(x);
                res.insert(num / x);
            }
        }

        res.remove(&1);

        res
    }
}
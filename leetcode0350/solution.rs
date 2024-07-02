use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        /// Time: O(n + m) | Space: O(n + m)
        /// 
        /// where n is the length of nums1
        /// and m is the length of nums2
        let mut map: HashMap<i32, i32> = HashMap::new();

        for num in nums1 {
            map.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);
        }

        let mut res: Vec<i32> = Vec::new();

        for num in nums2 {
            map.entry(num).and_modify(|cnt| {
                if *cnt > 0 {
                    res.push(num);
                    *cnt -= 1;
                }
            });
        }

        res
    }
}
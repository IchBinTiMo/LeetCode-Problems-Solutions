use std::collections::HashSet;

impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {

        let set1: HashSet<i32> = nums1.iter().map(|&val| val).collect();
        let set2: HashSet<i32> = nums2.iter().map(|&val| val).collect();

        if set1.is_disjoint(&set2) {
            return -1;
        }

        for &num in nums1.iter() {
            if nums2.contains(&num) {
                return num;
            }
        }

        -1
    }
}
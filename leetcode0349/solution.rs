impl Solution {
    pub fn intersection(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut current: i32 = i32::MIN;
        let mut i: usize = 0;
        let mut j: usize = 0;

        let mut res: Vec<i32> = Vec::new();

        nums1.sort_unstable();
        nums2.sort_unstable();

        while i < nums1.len() && j < nums2.len() {
            if nums1[i] == nums2[j] {
                if nums1[i] > current {
                    current = nums1[i];
                    res.push(current);
                }
                i += 1;
                j += 1;
            } else if nums1[i] > nums2[j] {
                j += 1;
            } else if nums1[i] < nums2[j] {
                i += 1;
            }
        }

        res
    }
}
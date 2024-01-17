use std::collections::HashSet;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut cnt: [i32; 2001] = [0; 2001];

        for &num in arr.iter() {
            cnt[(num + 1000) as usize] += 1;
        }

        let mut occ: HashSet<i32> = HashSet::new();

        for &c in cnt.iter() {
            if c == 0 {
                continue;
            }

            if !occ.insert(c) {
                return false;
            }
        }

        true
    }
}
use std::collections::HashMap;

impl Solution {
    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut hm: HashMap<i32, i32> = HashMap::new();

        for range in ranges {
            let (l, r) = (range[0], range[1]);
            for n in l..=r {
                *hm.entry(n).or_insert(0) += 1;
            }
        }

        for n in left..=right {
            if hm.get(&n).unwrap_or(&0) == &0 {
                return false;
            }
        }
        true
    }
}
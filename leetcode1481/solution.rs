use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, mut k: i32) -> i32 {
        let mut ht: HashMap<i32, i32> = HashMap::new();

        for &num in arr.iter() {
            ht.entry(num).and_modify(|val| *val += 1).or_insert(1);
        }

        let mut keys: Vec<i32> = ht.keys().map(|k| *k).collect();

        keys.sort_unstable_by_key(|key| -ht.get(key).unwrap());

        while k > 0 {
            let key = keys.last().unwrap();
            if let Some(&cnt) = ht.get(key) {
                if k >= cnt {
                    keys.pop();
                }
                k -= cnt;
            }
        }

        keys.len() as i32
    }
}
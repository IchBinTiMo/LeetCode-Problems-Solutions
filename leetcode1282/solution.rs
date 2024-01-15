use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ht: HashMap<i32, Vec<i32>> = HashMap::new();

        let mut ans: Vec<Vec<i32>> = vec![];

        for (idx, size) in group_sizes.iter().enumerate() {
            
            ht.entry(*size).or_insert(Vec::new()).push(idx as i32);

            if let Some(v) = ht.get_mut(size) {
                if v.len() as i32 == *size {
                    ans.push(v.to_vec());
                    v.clear();
                }
            }


        }

        ans
    }
}
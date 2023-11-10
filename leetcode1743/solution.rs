use std::collections::HashMap;

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        if adjacent_pairs.len() == 1 {
            return adjacent_pairs[0].clone();
        }

        let mut ans: Vec<i32> = Vec::new();

        let mut neighbors: HashMap<i32, Vec<i32>> = HashMap::new();

        for pair in adjacent_pairs {
            let l = pair[0];
            let r = pair[1];
            (*neighbors.entry(l).or_insert(vec![])).push(r);
            (*neighbors.entry(r).or_insert(vec![])).push(l);
        }

        let mut current = -1;
        let mut prev = -2;

        for key in neighbors.keys() {
            if neighbors.get(key).unwrap().len() == 1 {
                current = *key;
            }
        }
        ans.push(current);
        while let Some(value) = neighbors[&current].iter().filter(|&&v| v != prev).next() {
            prev = current;
            current = *value;
            ans.push(current);
        }

        ans
    }
}
use std::collections::HashSet;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut set: HashSet<&String> = HashSet::new();

        for path in &paths {
            set.insert(&path[0]);
        }

        for path in &paths {
            if !set.contains(&path[1]) {
                return String::from(&path[1]);
            }
        }

        String::new()
    }
}

// use std::collections::HashMap;

// impl Solution {
//     pub fn dest_city(paths: Vec<Vec<String>>) -> String {
//         let mut map: HashMap<String, i32> = HashMap::new();

//         for i in 0..paths.len() {
//             let (from, to) = (paths[i][0].clone(), paths[i][1].clone());
//             *map.entry(from.clone()).or_insert(0) += 1;
//             *map.entry(to.clone()).or_insert(0) -= 1;

//             if *map.get(&from).unwrap() == 0 {
//                 map.remove(&from);
//             }

//             if *map.get(&to).unwrap() == 0 {
//                 map.remove(&to);
//             }
//         }

//         for key in map.keys() {
//             if map[key] < 0 {
//                 return String::from(key);
//             }
//         }

//         String::new()
//     }
// }
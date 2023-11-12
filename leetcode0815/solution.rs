use std::collections::HashSet;
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut dist = vec![i16::MAX - 1; 1000001];
        dist[source as usize] = 0;
        let mut visited = HashSet::new();
        let mut count = 0;
        
        while visited.len() < routes.len() && count < routes.len() {
            for bus in 0..routes.len() {
                if visited.contains(&bus) {
                    continue;
                }
                let mut mindist = i16::MAX - 1;
                for i in routes[bus].iter() {
                    mindist = mindist.min(dist[*i as usize] + 1);
                }
                for i in routes[bus].iter() {
                    dist[*i as usize] = dist[*i as usize].min(mindist);
                }
                if mindist < (i16::MAX - 1) {
                    visited.insert(bus);
                }
            }
            count += 1;
        }
        if dist[target as usize] == (i16::MAX - 1) {
            -1
        } else {
            dist[target as usize] as i32
        }
    }
}
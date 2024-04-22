use std::collections::VecDeque;

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        /// BFS
        /// 
        /// Time: O(n) | Space: O(n)
        /// 
        /// it is a shortest path problem that there are 10000 nodes (0000 - 9999 except deadends) in the graph,
        /// and there is an edge between each pair of nodes if they differ in only one digit and the digit differs by 1
        let deadends: Vec<usize> = deadends.iter().map(|s| s.parse().unwrap()).collect();
        let target: usize = target.parse().unwrap();

        let mut visited: Vec<bool> = vec![false; 10000];

        let mut queue: VecDeque<(usize, i32)> = VecDeque::new();

        for &deadend in deadends.iter() {
            visited[deadend] = true;
        }

        queue.push_back((0000, 0));

        while let Some((current_lock, count)) = queue.pop_front() {

            if visited[current_lock] {
                continue;
            } else {
                if current_lock == target {
                    return count;
                } else {
                    visited[current_lock] = true;
                    for diff in [1000, 0100, 0010, 0001] {
                        let current_slot: usize = current_lock / diff % 10;
                        let plus_1: usize = (current_lock - diff * current_slot) + (diff * ((current_slot + 1) % 10)); // add 1 to current slot
                        let minus_1: usize = (current_lock - diff * current_slot) + (diff * ((current_slot + 9) % 10)); // subtract 1 from current slot

                        queue.push_back((plus_1, count + 1));
                        queue.push_back((minus_1, count + 1));
                    }
                }
            }
        }

        -1
    }
}
/*
Solution: Binary Heap

Time: O(n log n) | Space: O(n)

Runtime: 21 ms | 80.00%
Memory: 2.96 MB | 100.00%

- n: length of 'times'
*/

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn smallest_chair(times: Vec<Vec<i32>>, target_friend: i32) -> i32 {
        let mut chair_cnt: i32 = 0;

        let mut seat: HashMap<i32, i32> = HashMap::new();

        let mut released: BinaryHeap<i32> = BinaryHeap::new();

        let mut timeline: Vec<(i32, bool, i32)> = Vec::new();

        for i in 0..times.len() {
            timeline.push((times[i][0], true, i as i32));
            timeline.push((times[i][1], false, i as i32));
        } 

        timeline.sort_unstable();

        for time in timeline.iter() {
            let now: i32 = time.0;
            let come: bool = time.1;
            let friend: i32 = time.2;

            if come {
                if let Some(chair_num) = released.pop() {
                    if friend == target_friend {
                        return -chair_num;
                    } else {
                        seat.insert(friend, -chair_num);
                    }
                } else {
                    if friend == target_friend {
                        return chair_cnt;
                    } else {
                        seat.insert(friend, chair_cnt);
                        chair_cnt += 1;
                    }
                }
            } else {
                if let Some(chair_num) = seat.remove(&friend) {
                    released.push(-chair_num);
                }
            }
        }

        0
    }
}
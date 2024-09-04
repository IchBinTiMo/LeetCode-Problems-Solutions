/*
Solution:

Time: O(n + k) | Space: O(k)

Runtime: 8 ms | Memory: 88.89 MB
Space: 3.04 MB | 11.11%

- n: length of 'commands'
- k: length of 'obstacles'
*/

use std::collections::HashSet;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {

        let obs: HashSet<(i32, i32)> = obstacles.iter().map(|o| (o[0], o[1])).collect::<HashSet<(i32, i32)>>();

        let mut res: i32 = 0;

        let mut x: i32 = 0;
        let mut y: i32 = 0;
        
        let mut dirs: Vec<i32> = vec![0, 1, 0, -1, 0];
        let mut cur_dir: usize = 0;


        for &com in commands.iter() {
            match com {
                -2 => cur_dir = (cur_dir + 3) % 4,
                -1 => cur_dir = (cur_dir + 1) % 4,
                _ => {
                    let dx: i32 = dirs[cur_dir];
                    let dy: i32 = dirs[cur_dir + 1];

                    for i in 1..=com {
                        if obs.contains(&(x + dx, y + dy)) {
                            break;
                        } else {
                            x += dx;
                            y += dy;
                        }
                    }

                    res = res.max(x * x + y * y);
                }
            }
        }

        res
    }
}
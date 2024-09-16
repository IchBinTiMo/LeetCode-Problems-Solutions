/*
Solution:

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 3.16 MB | 62.50%

- n: length of 'time_points'
*/

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut visited: [bool; 1440] = [false; 1440];

        for tp in time_points.iter() {
            let time: i32 = Self::time_to_minute(tp);

            if visited[time as usize] {
                return 0;
            }

            visited[time as usize] = true;
        }

        let mut res: usize = usize::MAX;

        let mut first: usize = 0;
        let mut prev: usize = 0;

        while !visited[prev] {
            first += 1;
            prev += 1;
        }

        for i in (first + 1)..1440 {
            if visited[i] {
                res = res.min(i - prev);
                prev = i;
            }
        }

        res = res.min(1440 + first - prev);

        res as i32
    }

    fn time_to_minute(time: &str) -> i32 {
        let h: i32 = time[0..2].parse().unwrap();
        let m: i32 = time[3..5].parse().unwrap();

        h * 60 + m
    }
}
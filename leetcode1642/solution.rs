impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut sum: i32 = 0;

        let mut diffs: Vec<i32> = Vec::new();

        for i in 1..heights.len() {
            let diff: i32 = if heights[i] - heights[i - 1] > 0 {
                heights[i] - heights[i - 1]
            } else {
                0
            };

            sum += diff;

            match diffs.binary_search(&diff) {
                Ok(pos) => diffs.insert(pos, diff),
                Err(pos) => diffs.insert(pos, diff)
            }

            let biggest: i32 = diffs[((diffs.len() as i32 - ladders).max(0) as usize)..diffs.len()].iter().sum();

            if sum - biggest > bricks {
                return (i - 1) as i32;
            }
        }

        heights.len() as i32 - 1
    }
}
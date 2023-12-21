impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|point| point[0]);

        points.windows(2).map(|w| w[1][0] - w[0][0]).max().unwrap()
    }
}
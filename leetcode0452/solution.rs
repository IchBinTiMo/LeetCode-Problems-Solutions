impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        // sort by end
        points.sort_unstable_by_key(|p| p[1]);

        let mut res: i32 = 0;

        // [start, end]
        let mut prev_overlap: Vec<i32> = vec![i32::MIN, i32::MAX];

        while !points.is_empty() {
            while let Some(point) = points.pop() {
                // calculate the overlap of current overlap and new point
                let overlap: Vec<i32> = vec![prev_overlap[0].max(point[0]), prev_overlap[1].min(point[1])];

                // if new point doesn't overlap with previous one
                if overlap[0] > overlap[1] {
                    // push new point back and reset previous overlap
                    prev_overlap = vec![i32::MIN, i32::MAX];
                    points.push(point);
                    break;
                }

                // update previous overlap
                prev_overlap = overlap;
            }

            res += 1;
        }

        res
    }
}
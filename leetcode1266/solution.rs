impl Solution {
    pub fn min_time_to_visit_all_points(mut points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut current: Option<Vec<i32>> = None;

        while points.len() > 0 {
            let point = points.remove(0);
            match current {
                Some(val) => {
                    ans += ((point[0] - val[0]).abs()).max((point[1] - val[1]).abs());
                    current = Some(point);
                },
                _ => current = Some(point)
            }
        }

        ans
    }
}
// impl Solution {
//     pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
//         let mut ans = 0;
//         let mut current: Option<Vec<i32>> = None;

//         for i in 0..points.len() {
//             let point = points[i].clone();
//             match current {
//                 Some(val) => {
//                     ans += ((point[0] - val[0]).abs()).max((point[1] - val[1]).abs());
//                     current = Some(point);
//                 },
//                 _ => current = Some(point)
//             }
//         }

//         ans
//     }
// }
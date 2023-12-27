use std::cmp::max;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {

        let mut tmp = colors.bytes().zip(needed_time.iter());

        let (mut prev_color, &(mut max_time)) = tmp.next().unwrap();

        let mut ans = max_time;

        while let Some((color, &time)) = tmp.next() {
            if color == prev_color {
                max_time = max(max_time, time);
            } else {
                ans -= max_time;
                prev_color = color;
                max_time = time;
            }

            ans += time
        }

        ans -= max_time;

        ans
    }
}

// impl Solution {
//     pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {

//         let mut ans = 0;

//         let mut prev: Option<(char, i32)> = None; // (color, time)>

//         for (color, &time) in colors.chars().zip(needed_time.iter()) {
//             match prev {
//                 Some((prev_color, prev_time)) => {
//                     if prev_color == color {
//                         prev = Some((color, prev_time.max(time)));
//                     } else {
//                         ans -= prev_time;
//                         prev = Some((color, time));
//                     }
//                 },
//                 _ => {
//                     prev = Some((color, time));
//                 }
//             }
//             ans += time;
//         }

//         ans -= prev.unwrap().1;

//         ans
//     }
// }

// // impl Solution {
// //     pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {

// //         let colors = colors.bytes().collect::<Vec<u8>>();
// //         let l = colors.len();
// //         let mut ans = 0;

// //         let mut left: usize = 0;
// //         let mut right: usize = 1;

// //         while left < l{
// //             while right < l && colors[right] == colors[left] {
// //                 right += 1;
// //             }

// //             let mut mx = i32::MIN;
// //             let sum: i32 = needed_time[left..right].iter().sum();

// //             while left < right {
// //                 mx = mx.max(needed_time[left]);
// //                 left += 1;
// //             }

// //             ans += sum - mx;
// //             right += 1;

// //         }

// //         ans
// //     }
// // }
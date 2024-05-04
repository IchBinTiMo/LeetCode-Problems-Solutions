impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        /// Two Pointers
        /// 
        /// Time: O(nlogn) | Space: O(1)
        people.sort_unstable();

        let n: usize = people.len();

        let mut left: usize = 0;
        let mut right: usize = n - 1;

        let mut res: i32 = 0;

        while right < n && left <= right {
            if people[left] + people[right] <= limit {
                left += 1;
            }
            right -= 1;

            res += 1;
        }

        res
    }
}

// impl Solution {
//     pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
//         /// Two Pointers
//         /// 
//         /// Time: O(nlogn) | Space: O(1)
//         people.sort_unstable();

//         let n: usize = people.len();

//         let mut left: usize = 0;
//         let mut right: usize = n - 1;

//         let mut res: i32 = 0;

//         while right < n && left <= right {
//             let mut sum: i32 = 0;
//             let mut count: i32 = 0;

//             while right < n && limit - sum >= people[right] && count < 2 {
//                 sum += people[right];
//                 right -= 1;
//                 count += 1;
//             }

//             while left < n && limit - sum >= people[left] && count < 2 {
//                 sum += people[left];
//                 left += 1;
//                 count += 1;
//             }

//             res += 1;
//         }

//         res
//     }
// }
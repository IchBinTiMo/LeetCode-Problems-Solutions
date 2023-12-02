impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = arr.len() - 1;

        while left < right {
            let mid = (left + right) / 2;
            
            match arr[mid] < arr[mid + 1] {
                true => left = mid + 1,
                _ => right = mid
            }
        }

        left as i32
    }
}
// impl Solution {
//     pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
//         let mut left: usize = 0;
//         let mut right: usize = arr.len() - 1;
//         let mut mid = (left + right) / 2;

//         loop {
//             mid = (left + right) / 2;
//             if arr[mid] > arr[mid + 1] && arr[mid] > arr[mid - 1] {
//                 break;
//             } else if arr[mid] > arr[mid + 1] {
//                 right = mid;
//             } else {
//                 left = mid
//             }
//         }

//         mid as i32
//     }
// }
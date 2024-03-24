impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        /// Floyd's Cycle Detection Algorithm
        /// 
        /// Time Complexity:     O(N)
        /// Space Complexity:    O(1)
        /// 
        /// Key: Treat the array as a linked list while the index represents the node and the value represents the next node
        let mut slow: i32 = nums[0];
        let mut fast: i32 = nums[0];

        // Find the intersection point of the two pointers
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];

            if slow == fast {
                break;
            }
        }

        // Find the entry point of the cycle
        slow = nums[0];

        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        slow
    }
}

// impl Solution {
//     pub fn find_duplicate(nums: Vec<i32>) -> i32 {
//         /// Binary Search
//         /// 
//         /// Time Complexity:     O(NlogN)
//         /// 
//         /// Space Complexity:    O(1)
//         /// 
//         /// Key: Treat the unsorted array as a sorted array
//         let mut left: usize = 0;
//         let mut right: usize = nums.len() - 1;

//         while left < right {
//             let mut count: usize = 0;
//             let mid: usize = left + (right - left) / 2;

//             // Count the number of elements that are less than or equal to mid
//             for i in 0..nums.len() {
//                 if nums[i] <= mid as i32 {
//                     count += 1;
//                 }
//             }

//             // If count is more than mid, then we need to search in the left part
//             if count > mid {
//                 right = mid;
//             } else { // If count is less than or equal to mid, then we need to search in the right part
//                 left = mid + 1;
//             }
//         }

//         left as i32
//     }
// }
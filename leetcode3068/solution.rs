impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        /// Time: O(n) | Space: O(1)
        let mut res: i64 = 0;

        let mut positive_count = 0;

        let mut max_negative: i64 = i64::MIN; // the maximum negative difference between the toggled value and the original value
        let mut min_positive: i64 = i64::MAX; // the minimum positive difference between the toggled value and the original value

        // iterate through the nums
        for &num in nums.iter() {
            let num = num as i64;
            let toggled = num ^ k as i64;

            if toggled - num >= 0 {
                // if the toggled value is greater than or equal to the original value
                // then we can add the toggled value to the res
                // and update the min_positive
                res += toggled;
                positive_count += 1;
                min_positive = min_positive.min(toggled - num);
            } else {
                // if the toggled value is less than the original value
                // then we add the original value to the res
                // and update the max_negative
                res += num;
                max_negative = max_negative.max(toggled - num);
            }
        }

        // Because there is always an even number of nodes that are toggled
        // so if we choose odd nodes to add its toggled value to the res
        // we need to consider the next maximum difference, which is max_negative
        // to decide whether we should toggle one more node 
        // or untoggled one more node
        if positive_count & 1 == 1 {
            if min_positive + max_negative > 0 {
                res += max_negative;
            } else {
                res -= min_positive;
            }
        }

        res
    }
}

// impl Solution {
//     pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
//         /// The value that the node is toggled odd times is the same as the value that the node is toggled one time.
//         /// The value that the node is toggled even times is the same as the value that the node is not toggled.
//         /// And in each operation, we have to choose 2 nodes to be toggled
//         /// so there will always be even nodes that are toggled
//         /// Time: O(nlogn) | Space: O(n)
//         let n: usize = nums.len();

//         // get the values of all nodes after even times XOR by k
//         let mut toggled: Vec<i32> = nums.iter().map(|&e| e ^ k).collect();

//         // the sum of all nodes with 0 nodes toggled
//         let mut start: i64 = nums.iter().fold(0, |acc, &x| acc + x as i64);

//         let mut current: i64 = 0;

//         // calculatte the difference between the toggled value and the original value
//         let mut diff: Vec<i64> = (0..n).map(|idx| toggled[idx] as i64 - nums[idx] as i64).collect();

//         // sort the difference in descending order
//         diff.sort_unstable_by_key(|e| -e);

//         let mut i: usize = 0;

//         while i < n {

//             // bound check
//             if diff.get(i + 1).is_some() {
//                 if diff[i] + diff[i + 1] < 0 {
//                     break;
//                 }
                
//                 current += diff[i] + diff[i + 1];
//             } else {
//                 if diff[i] < 0 {
//                     break;
//                 }
//             }

//             // since there will always be even nodes that are toggled
//             // so we can increment i by 2
//             i += 2;
//         }

//         current + start
//     }
// }
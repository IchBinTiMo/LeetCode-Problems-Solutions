impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        /// Time: O(n) | Space: O(1)
        /// where n is the length of nums
        
        // xor all the numbers
        let xor: i32 = nums.iter().fold(0, |acc, &x| acc ^ x);

        // find the rightmost set bit
        // which means the two elements we need are different at "zeros + 1" bit
        let zeros: i32 = xor.trailing_zeros() as i32;

        let mut res: Vec<i32> = vec![0, 0];

        // Therefore, we can split the numbers into two groups
        // with the "zeros + 1" of all the numbers in one group is set
        // and the "zeros + 1" of all the numbers in the other group is not set
        for &num in nums.iter() {
            if (num >> zeros) & 1 == 0  {
                res[0] ^= num;
            } else {
                res[1] ^= num;
            }
        }

        res
    }
}

// impl Solution {
//     pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
//         let mut tmp: i32 = 0;

//         for &num in nums.iter() {
//             tmp ^= num;
//         }

//         let mut shift: i32 = 0;

//         loop {
//             if tmp & 1 == 0 {
//                 shift += 1;
//                 tmp >>= 1;
//             } else {
//                 break;
//             }
//         }

//         let mut first: i64 = 0;
//         let mut second: i64 = 0;

//         for num in nums.iter().map(|&x| x as i64) {
//             if num & (1 << shift) == 0 {
//                 first ^= num;
//             } else {
//                 second ^= num;
//             }
//         }

//         vec![first as i32, second as i32]
//     }
// }
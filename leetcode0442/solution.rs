impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        /// Floyd's Cycle Detection Algorithm
        /// 
        /// Time Complexity:     O(N)
        /// Space Complexity:    O(1)
        /// 
        /// Key: Treat the array as a linked list while the index represents the node and the value represents the next node
        let mut res: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            let num = nums[i];

            // If the next node is negative, it means the node has been visited, so it is a duplicate
            if nums[num.abs() as usize - 1] < 0 {
                res.push(num.abs());
            }
            // Set next node to negative to mark the node as visited
            nums[num.abs() as usize - 1] *= -1;
        }

        res
    }
}
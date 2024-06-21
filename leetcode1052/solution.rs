impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        /// Sliding Window
        /// 
        /// Time: O(n) | Space: O(1)
        /// where n is the length of customers
        let n: usize = customers.len();

        let mut left: usize = 0; // left pointer of bookstore owner using technique to avoid being grumpy

        let mut is_grumpy: i32 = 0; // maximum customers can be satisfied in grumpy minutes
        let mut not_grumpy: i32 = 0; //customers can be satisfied in not grumpy minutes

        let mut current: i32 = 0; // customers satisfied in current grumpy minutes

        for right in 0..n {
            if right - left >= minutes as usize {
                // shrink the window if the current window is larger than the maximum minutes that technique can be used
                if grumpy[left] == 1 {
                    current -= customers[left];
                }
                left += 1;
            }

            if grumpy[right] == 0 {
                not_grumpy += customers[right];
            } else {
                current += customers[right];
                is_grumpy = is_grumpy.max(current); // update the maximum satisfied customers in grumpy minutes
            }
        }

        is_grumpy + not_grumpy
    }
}
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        /// Time: O(n) | Space: O(1)
        /// 
        /// where n is the number of bottles
        let mut res: i32 = num_bottles;

        while num_bottles >= num_exchange {
            res += num_bottles / num_exchange;
            num_bottles = num_bottles / num_exchange + num_bottles % num_exchange;
        }

        res
    }
}
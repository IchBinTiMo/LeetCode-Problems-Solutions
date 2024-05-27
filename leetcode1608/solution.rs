impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        /// Time: O(n) | Space: O(n)
        /// 
        /// where n is length of `nums`
        let mut freqs: Vec<i32> = vec![0; 1001];

        // Count the frequency of each number
        for &num in nums.iter() {
            freqs[num as usize] += 1;
        }

        // the count of remained numbers
        let mut remained: i32 = nums.len() as i32;

        for i in 1..=1000 {
            remained -= freqs[i - 1];

            if remained == i as i32 {
                return remained;
            }
        }

        -1
    }
}
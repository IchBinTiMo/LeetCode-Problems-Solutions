impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        /// Time: O(n) | Space: O(1)
        /// where n is the length of nums
        let mut freqs: [i32; 3] = [0; 3];

        for &num in nums.iter() {
            freqs[num as usize] += 1;
        }

        // the index of the next number to be filled
        let mut idx: usize = 0;

        for i in 0..freqs.len() {
            for _ in 0..freqs[i] {
                nums[idx] = i as i32;
                idx += 1;
            }
        }
    }
}
impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut freq: [i32; 3] = [0; 3];

        for &num in nums.iter() {
            freq[num as usize] += 1;
        }

        let mut idx: usize = 0;

        for i in 0..=2 {
            for _ in 0..freq[i] {
                nums[idx] = i as i32;
                idx += 1;
            }
        }
    }
}
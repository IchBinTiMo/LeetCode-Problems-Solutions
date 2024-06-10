impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        /// Counting Sort
        /// 
        /// Time O(n) | Space O(n)
        /// where n is the length of heights
        let mut freqs: Vec<i32> = vec![0; 100];

        for &height in heights.iter() {
            freqs[(height - 1) as usize] += 1;
        }

        let mut expected: Vec<i32> = Vec::new();

        for i in 0..freqs.len() {
            for _ in 0..freqs[i] {
                expected.push((i + 1) as i32);
            }
        }

        let mut res: i32 = 0;

        for (&height, &exp) in heights.iter().zip(expected.iter()) {
            if height != exp {
                res += 1;
            }
        }

        res
    }
}
impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        /// Move all extra x to x + 1
        /// 
        /// Time O(n + 200001) | Space O(200001)
        /// where n is the length of nums
        
        let mut freqs: [i32; 200001] = [0; 200001];
        let mut min: i32 = i32::MAX; // the index we should start from

        for &num in nums.iter() {
            freqs[num as usize] += 1;
            min = min.min(num);
        }

        let mut res: i32 = 0;

        let mut i: usize = min as usize;
        let mut cnt: usize = 0; // to count how many numbers we have seen, in order to know when to stop

        while cnt < nums.len() {
            let mut freq: i32 = freqs[i];

            cnt += if freq == 0 {0} else {1};

            if freq > 1 {
                freqs[i + 1] += freq - 1;
                res += freq - 1;
            }

            i += 1;
        }

        res
    }
}
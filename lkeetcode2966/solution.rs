impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let mut ans: Vec<Vec<i32>> = Vec::new();

        let mut current: usize = 0;

        while current < nums.len() {
            let mut tmp: Vec<i32> = Vec::new();

            if nums[current + 2] - nums[current] > k {
                return Vec::new();
            }

            for i in 0..3 {
                let i = i as usize;

                tmp.push(nums[current + i]);
            }

            ans.push(tmp);

            current += 3;
        }

        ans
    }
}
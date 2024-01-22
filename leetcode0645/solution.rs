impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut map: Vec<i32> = vec![0; nums.len()];

        let mut ans: Vec<i32> = vec![0, 0];


        for &num in nums.iter() {
            map[(num - 1) as usize] += 1;
        }

        for i in 0..map.len() {
            match map[i] {
                0 => ans[1] += (i + 1) as i32,
                2 => ans[0] += (i + 1) as i32,
                _ => {}
            }
        }

        ans
        
    }
}
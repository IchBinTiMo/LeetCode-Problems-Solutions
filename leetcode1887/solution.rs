impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut idx: Vec<i32> = vec![0; 50000];
        for num in nums.iter() {
            idx[(num - 1) as usize] += 1;
        }
        idx = idx.iter().filter(|x| *x != &0).map(|x| *x).collect();
        let mut lg: usize = idx.len() - 1;
        let mut ans: i32 = 0;

        while lg > 0 {
           ans += idx[lg] * lg as i32;
           lg -= 1;
        }
        ans
    }
}
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut nums: Vec<(u32, i32)> = arr.iter().map(|x| (x.count_ones(), *x)).collect();
        nums.sort();
        let ans: Vec<i32> = nums.iter().map(|x| x.1).collect();
        ans
    }
}
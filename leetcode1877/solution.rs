impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        let mut count: Vec<i32> = vec![0;100000];
        let mut mn: i32 = i32::MAX;
        let mut mx: i32 = i32::MIN;
        for &num in nums.iter() {
            count[(num - 1) as usize] += 1;
            mn = mn.min(num);
            mx = mx.max(num);
        }

        let mut left: usize = 0;
        let mut right: usize = 99999;
        let mut ans = i32::MIN;

        while left <= right {
            if count[left] <= 0 {
                left += 1;
            } else if count[right] <= 0 {
                right -= 1;
            } else {
                ans = ans.max((left + right) as i32 + 2);
                let cnt = count[left].min(count[right]);
                count[left] -= cnt;
                count[right] -= cnt;
            }
        }
        ans
    }
}
// impl Solution {
//     pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
//         let mut nums: Vec<i32> = nums;
//         nums.sort_unstable();
//         let mut ans = 0;
//         for i in 0..(nums.len() / 2) {
//             ans = ans.max(nums[i] + nums[nums.len() - 1 - i]);
//         }
//         ans
//     }
// }
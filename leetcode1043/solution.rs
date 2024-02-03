impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let n: usize = arr.len();

        let k: usize = k as usize;

        let mut dp: Vec<i32> = vec![0; n + 1];

        for i in 1..=n {
            let mut mx: i32 = i32::MIN;
            for j in 1..=k {
                if i as i32 - j as i32 >= 0{
                    mx = mx.max(arr[i - j]);
                    dp[i] = dp[i].max(dp[i - j] + mx * j as i32);
                } else {
                    break;
                }
            }
        }

        dp[n]
    }
}
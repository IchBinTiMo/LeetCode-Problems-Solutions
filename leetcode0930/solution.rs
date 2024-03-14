impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {

        let n: usize = nums.len();

        // vector to save the indices prefix sum changes inclusively
        let mut stops: Vec<i32> = Vec::new();

        stops.push(-1);

        // prefix sum
        let mut acc: i32 = 0;

        let mut res: i32 = 0;

        // calculate the prefix sum and save the indices
        for i in 0..n {
            if nums[i] == 1 {
                acc += 1;
                stops.push(i as i32);
            }
        }

        stops.push(n as i32);

        // corner case (goal = 0)
        if goal == 0 {
            for i in 1..stops.len() {
                let zeros: i32 = stops[i] - stops[i - 1];
                res += zeros * (zeros - 1) / 2;
            }

            return res;
        }

        let goal = goal as usize;


        // calculate the number of subarrays
        for i in 1..(stops.len() - goal) {
            let end: usize = i + goal - 1;

            res += (stops[i] - stops[i - 1]) * (stops[end + 1] - stops[end]);
        }

        res as i32
    }
}
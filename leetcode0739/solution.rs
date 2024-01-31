impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n: usize = temperatures.len();

        let mut stack: Vec<usize> = Vec::new();

        let mut ans: Vec<i32> = vec![0; n];

        for i in 0..n {
            while let Some(&idx) = stack.last() {
                if temperatures[idx] >= temperatures[i] {
                    break;
                } else {
                    ans[idx] = (i - idx) as i32;
                    stack.pop();
                }
            }

            stack.push(i);
        }

        ans
    }
}
impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {

        let colors = colors.bytes().collect::<Vec<u8>>();
        let l = colors.len();
        let mut ans = 0;

        let mut left: usize = 0;
        let mut right: usize = 1;

        while left < l{
            while right < l && colors[right] == colors[left] {
                right += 1;
            }

            let mut mx = i32::MIN;
            let sum: i32 = needed_time[left..right].iter().sum();

            while left < right {
                mx = mx.max(needed_time[left]);
                left += 1;
            }

            ans += sum - mx;
            right += 1;

        }

        ans
    }
}
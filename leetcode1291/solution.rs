impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        let mut base: i32 = 123456789;

        let mut left: i32 = 1000000000;

        while left > 0 {
            let mut right: i32 = left / 100;
            while right > 0 {
                let tmp: i32 = base % left / right;
                if tmp >= low && tmp <= high {
                    ans.push(tmp)
                }
                right /= 10;
            }
            left /= 10;
        }
        
        ans.sort_unstable();

        ans
    }
}
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {

        let n: usize = height.len();

        if n <= 2 {
            return 0;
        }

        let mut ans: i32 = 0;

        let mut maxL: i32 = height[0];
        let mut maxR: i32 = height[n - 1];

        let mut left: usize = 1;
        let mut right: usize = n - 2;

        while left <= right {
            if maxL < maxR {
                if height[left] > maxL {
                    maxL = height[left];
                } else {
                    ans += maxL - height[left];
                }

                left += 1;
            } else {
                if height[right] > maxR {
                    maxR = height[right];
                } else {
                    ans += maxR - height[right];
                }

                right -= 1;
            }
        }

        ans
    }
}
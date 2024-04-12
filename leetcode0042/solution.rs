impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        /// Time O(N) | Space O(1)
        let n: usize = height.len();

        if n <= 2 {
            return 0;
        }

        let mut ans: i32 = 0;

        let mut maxL: i32 = height[0]; // max height of left wall
        let mut maxR: i32 = height[n - 1]; // max height of right wall

        let mut left: usize = 1;
        let mut right: usize = n - 2;

        while left <= right {
            // compare max height of left wall and right wall
            if maxL < maxR {
                // if current height of left wall is higher then update maxL, otherwise increase ans by maxL - height[left]
                if height[left] > maxL {
                    maxL = height[left];
                } else {
                    ans += maxL - height[left];
                }

                left += 1;
            } else { 
                // if current height of right wall is higher then update maxL, otherwise increase ans by maxR - height[right]
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
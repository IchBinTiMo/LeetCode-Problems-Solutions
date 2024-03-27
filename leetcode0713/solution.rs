impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        
        let n: usize = nums.len();

        let mut res: usize = 0;

        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut product: i32 = 1;

        while right < n {
            product *= nums[right];
            right += 1;
            while left < right && product >= k {
                product /= nums[left];
                left += 1;
            }

            res += right - left;
        }

        res as i32
    }
}
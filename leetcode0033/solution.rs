impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        if nums.len() == 1 {
            return -1 + nums.contains(&target) as i32;
        }

        let (mut left, mut right) = (0 as usize, nums.len() - 1);

        let mut head = nums.len() - 1;
        
        match nums[left] < nums[right] {
            false => {
                while left < right {
                    let mid = (left + right) / 2;
                    match nums[mid] < nums[right] {
                        true => right = mid,
                        false => left = mid + 1
                    }
                }
                head = left;
                if target >= nums[0] {
                    left = 0;
                    right = head - 1;
                } else {
                    left = head;
                    right = nums.len() - 1;
                }
            }
            _ => {
                    head = 0;
                    left = 0;
                    right = nums.len() - 1;
                }
        }

        let mut ans = 0;

        while left <= right && right < nums.len() {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid - 1;
            } else {
                return mid as i32;
            }
        }

        -1
    }
}


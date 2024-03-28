use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let n: usize = nums.len();

        let mut res: usize = 0;

        let mut left: usize = 0;
        let mut right: usize = 0;

        let mut ht: HashMap<i32, i32> = HashMap::new();

        while right < n {
            ht.entry(nums[right])
                .and_modify(|e| *e += 1)
                .or_insert(1);

            if let Some(&freq) = ht.get(&nums[right]) {
                // if 'right' is at the end of the array and the frequencies of all the elements are not greater than k
                if freq <= k && right == n - 1 {
                    res = res.max(right - left + 1);
                }
                
                // if 'freq' is greater than 'k' then shrink the window
                while freq > k {
                    res = res.max(right - left);
                    ht.entry(nums[left])
                        .and_modify(|e| *e -= 1);
                    
                    // if the frequency of 'nums[left]' is 0 then remove it from the hashmap
                    if let Some(&out) = ht.get(&nums[left]) {
                        if out == 0 {
                            ht.remove(&nums[left]);
                        }
                    }
                    left += 1;

                    // if the element just popped is equal to the element at 'right' then break
                    if nums[left - 1] == nums[right] {
                        break;
                    }
                }
            }
            right += 1;
        }

        res as i32
    }
}
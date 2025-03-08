/*
Solution:

Time: O(n) | Space: O(n)

Runtime: 3 ms | 47.36%
Memory: 4.37 MB | 8.92%

- n: length of 'nums'
*/

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut less: Vec<i32> = Vec::new();
        let mut equal: i32 = 0;
        let mut great: Vec<i32> = Vec::new();

        for &num in nums.iter() {
            if num < pivot {
                less.push(num);
            } else if num == pivot {
                equal += 1;
            } else{
                great.push(num);
            }
        }

        let mut res: Vec<i32> = Vec::new();

        for num in less {
            res.push(num);
        }

        for _ in 0..equal {
            res.push(pivot);
        }

        for num in great {
            res.push(num);
        }

        res
    }
}
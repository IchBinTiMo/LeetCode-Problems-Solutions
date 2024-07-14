use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        /// Time: O(n) | Space: O(n)
        /// 
        /// where 'n' is the length of 'n'
        let mut visited: HashSet<i32> = HashSet::new();

        while n != 1 {
            let mut sum: i32 = 0;

            while n != 0 {
                let rem = n % 10;
                sum += rem * rem;
                n /= 10;
            }

            if visited.contains(&sum) {
                return false;
            } else {
                visited.insert(sum);
            }

            n = sum;
        }

        true
        
    }
}
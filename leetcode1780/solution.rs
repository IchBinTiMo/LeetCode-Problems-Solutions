/*
Solution: 

Time: O(1) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 51.11%
*/

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n != 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}

/*
Solution: 

Time: O(n) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 51.11%
*/

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        let mut visited: i32 = 0;

        let mut current: i32 = 1;
        let mut power: i32 = 0;

        while current * 3 <= n {
            current *= 3;
            power += 1;
        }

        while current > 0 {
            if visited & (1 << power) == 0 && n >= current {
                n -= current;
                visited &= 1 << power;
            }

            current /= 3;
            power -= 1;

        }

        n == 0
    }
}
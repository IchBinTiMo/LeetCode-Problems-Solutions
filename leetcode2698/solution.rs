/*
Solution: 

Time: O(n) | Space: O(1)

Runtime: 9 ms | 60.00%
Memory: 2.16 MB | 100.00%


*/

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut res: i32 = 0;

        for i in 1..=n {
            if Self::is_valid(i) {
                res += i * i;
            }
        }

        res
    }

    fn is_valid(n: i32) -> bool {
        let mut flag: bool = false;

        Self::backtrack(&mut flag, 0, n * n, n);

        return flag;
    }

    fn backtrack(flag: &mut bool, current: i32, remain: i32, target: i32) {
        if remain == 0 && current == target {
            *flag = true;
        } else {
            let mut base: i32 = 10;

            while remain / base > 0 {
                Self::backtrack(flag, current + remain % base, remain / base, target);
                base *= 10;

                if *flag {
                    return;
                }
            }

            if remain / base == 0 && remain > 0 {
                Self::backtrack(flag, current + remain % base, remain / base, target);
            }
        }
    }
}
/*
Solution: Bit Manipulation

Time: O(1) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.26 MB | 50.00%
*/

impl Solution {
    pub fn minimize_xor(mut num1: i32, num2: i32) -> i32 {
        let cnt_1: i32 = num1.count_ones() as i32;
        let mut cnt_2: i32 = num2.count_ones() as i32;

        let mut res: i32 = 0;

        if cnt_1 == cnt_2 {
            return num1;
        } else if cnt_1 > cnt_2 {
            let mut current: i32 = 1 << 30;

            while cnt_2 > 0 {
                if num1 & current == current {
                    res |= current;
                    cnt_2 -= 1;
                }

                current >>= 1;
            }
        } else {
            let mut current: i32 = 1;

            res = num1;
            cnt_2 -= cnt_1;

            while cnt_2 > 0 {
                if num1 & current == 0 {
                    res |= current;
                    cnt_2 -= 1;
                }

                current <<= 1;
            }
        }

        return res;
    }
}
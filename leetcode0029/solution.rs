impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == divisor {
            return 1;
        }
        let neg = (dividend < 0) ^ (divisor < 0);
        let mut ans = 0;
        let mut dvd = dividend.abs() as u32;
        let mut dvr = divisor.abs() as u32;


        while dvd >= dvr {
            let mut tmp = 0;
            while dvd > (dvr << (tmp + 1)) {
                tmp += 1;
            }
            ans += 1 << tmp;
            dvd -= dvr << tmp;
        }

        if ans == 1 << 31 && !neg {
            return i32::MAX;
        }

        if neg {
            return -ans;
        } else {
            return ans;
        }
    }
}
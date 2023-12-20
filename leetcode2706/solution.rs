impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut mn = i32::MAX;
        let mut smn = i32::MAX;

        for &price in prices.iter() {
            if price < mn {
                smn = mn;
                mn = price;
            } else if price < smn {
                smn = price;
            }
        }

        return match mn + smn > money {
            true => money,
            _ => money - mn - smn
        };
    }
}
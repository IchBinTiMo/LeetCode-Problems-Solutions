impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();

        let mut dp: Vec<i32> = vec![i32::MAX; (amount + 1) as usize];

        dp[0] = 0;

        for i in 1..=amount {
            if coins.contains(&i) {
                dp[i as usize] = 1;
            } else {
                for &coin in coins.iter() {
                    if coin >= i {
                        break;
                    }

                    if dp[(i - coin) as usize] == i32::MAX {
                        continue;
                    }

                    dp[i as usize] = dp[i as usize].min(dp[(i - coin) as usize] + 1);
                }
            }
        }

        return if dp[amount as usize] == i32::MAX {
            -1
        } else {
            dp[amount as usize]
        }
    }
}
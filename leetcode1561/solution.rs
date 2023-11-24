impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        let mut piles = piles;

        piles.sort_unstable();
        let n = piles.len();
        piles[n / 3 ..].iter().step_by(2).sum()
    }
}
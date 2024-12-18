/*
Solution: Monotonic Stack

TIme: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.28 MB | 44.44%

- n: length of 'prices'
*/

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let n: usize = prices.len();

        let mut ms: Vec<usize> = Vec::new();

        let mut res: Vec<i32> = vec![0; n];

        ms.push(0);

        for j in 1..n {
            while let Some(i) = ms.pop() {
                if prices[j] <= prices[i] {
                    res[i] = prices[i] - prices[j];
                } else {
                    ms.push(i);
                    break;
                }
            }

            ms.push(j);
        }

        for &i in ms.iter() {
            res[i] = prices[i];
        }

        res
    }
}
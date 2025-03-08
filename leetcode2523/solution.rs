/*
Solution: 

Time: O(n ^ 2) | Space: O(n)

Runtime: 1057 ms | 100.00%
Memory: 2.58 MB | 100.00% MB

- n: 'right'
*/

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut pq: Vec<i32> = Vec::new();

        let mut res: Vec<i32> = Vec::new();

        for i in left..=right {
            if Self::is_prime(i) {
                while !pq.is_empty() && *pq.last().unwrap() >= i {
                    pq.pop();
                }

                pq.push(i);

                let n: usize = pq.len();

                if n >= 2 && 
                    (res.is_empty() || pq[n - 1] - pq[n - 2] < res[1] - res[0]) {
                        res = vec![pq[n - 2], pq[n - 1]];
                }
            }
        }

        if res.is_empty() {
            vec![-1, -1]
        } else {
            res
        }
    }

    fn is_prime(n: i32) -> bool {
        if n == 1 {
            return false;
        } else {
            let mut i: i32 = 2;

            while i * i <= n {
                if n % i == 0 {
                    return false;
                } else {
                    i += 1;
                }
            }
        }

        true
    }
}
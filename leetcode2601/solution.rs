/*
Solution: Greedy

Time: O(n) | Space: O(1)

Runtime: 24 ms | 100.00%
Memory: 2.14 MB | 100.00%

- n: length of 'nums'
*/

impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        let primes: Vec<i32> = Self::get_prime();

        for i in 0..nums.len() {
            let mut idx: usize = primes.partition_point(|x| x < &nums[i]);

            if idx >= primes.len() {
                idx -= 1;
            }

            while nums[i] <= primes[idx] {
                idx -= 1;
                if idx > primes.len() {
                    return false;
                }
            }

            if i == 0 {
                nums[i] -= primes[idx];
            } else {
                while nums[i] - primes[idx] <= nums[i - 1] {
                    if idx == 0 {
                        return false;
                    }
                    idx -= 1;
                }
                nums[i] -= primes[idx];
            }
        }

        true
    }

    fn get_prime() -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        res.push(0);

        for i in 2..1001 {
            let mut j: i32 = 1;

            let mut cnt: i32 = 0;
            
            while j * j <= i {
                if i % j == 0 {
                    cnt += 1;
                }

                j += 1;
            }

            if cnt == 1 {
                res.push(i);
            }
        }

        res
    }
}
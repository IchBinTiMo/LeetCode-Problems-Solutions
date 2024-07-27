/*
Solution 1:

Floyd-Warshall Algorithm

Time: O(n + C) | Space: O(n ^ 2)

- n: length of edges
- C: the maximum value of C is (26 ^ 3 + 25 ^ 2)
*/

impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let mut costs: Vec<Vec<i64>> = vec![vec![-1; 26]; 26];

        // the cost is 0 if the letter doesn't change
        for i in 0..26 {
            costs[i][i] = 0;
        }

        // initialize costs
        for i in 0..cost.len() {
            let from: usize = ((original[i] as u8) - ('a' as u8)) as usize;
            let to: usize = ((changed[i] as u8) - ('a' as u8)) as usize;

            costs[from][to] = if costs[from][to] == -1 {
                cost[i] as i64
            } else {
                costs[from][to].min(cost[i] as i64)
            };
        }

        // floyd-warshall
        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    if costs[i][k] == -1 || costs[k][j] == -1 {
                        continue;
                    }
                    
                    costs[i][j] = if costs[i][j] == -1 {
                        costs[i][k] + costs[k][j]
                    } else {
                        costs[i][j].min(costs[i][k] + costs[k][j])
                    };
                }
            }
        }

        let mut res: i64 = 0;

        for (from, to) in source.bytes().zip(target.bytes()) {
            let from: usize = (from - ('a' as u8)) as usize;
            let to: usize = (to - ('a' as u8)) as usize;

            if costs[from][to] == -1 {
                return -1;
            }

            res += costs[from][to];
        }

        res
    }
}
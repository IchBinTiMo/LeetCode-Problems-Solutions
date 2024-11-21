/*
Solution: 

Time: O(m * n) | Space: O(m * n)

Runtime: 15 ms | 100.00%
Memory: 10.16 MB | 100.00%
*/

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let mut res: i32 = m * n;

        let m: usize = m as usize;
        let n: usize = n as usize;

        let mut visited: Vec<Vec<i32>> = vec![vec![0; n]; m];

        let mut set: HashSet<(usize, usize)> = HashSet::new();

        for g in guards.iter() {
            visited[g[0] as usize][g[1] as usize] = 1;
        }

        for w in walls.iter() {
            visited[w[0] as usize][w[1] as usize] = 1;
        }

        for g in guards.iter() {
            let r: usize = g[0] as usize;
            let c: usize = g[1] as usize;

            for i in (0..r).rev() {
                if visited[i][c] == 1 {
                    break;
                }

                visited[i][c] = 2;
            }

            for i in (r + 1)..m {
                if visited[i][c] == 1 {
                    break;
                }

                visited[i][c] = 2;
            }

            for j in (0..c).rev() {
                if visited[r][j] == 1 {
                    break;
                }

                visited[r][j] = 2;
            }

            for j in (c + 1)..n {
                if visited[r][j] == 1 {
                    break;
                }

                visited[r][j] = 2;
            }
        }

        for r in visited.iter() {
            for &v in r.iter() {
                res -= ((v != 0) as i32) ^ 0;
            }
        }

        res
    }
}
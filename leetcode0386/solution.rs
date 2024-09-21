/*
Solution: DFS

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.61 MB | 72.73%
*/

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        Self::dfs(&mut res, 0, n);

        res
    }

    fn dfs(res: &mut Vec<i32>, path: i32, n: i32) {
        if path > n {
            return;
        }

        if path <= n && path > 0 {
            res.push(path);
        }

        let start: i32 = if path == 0 {1} else {0};

        for i in start..10 {
            Self::dfs(res, path * 10 + i, n);
        }
    }
}
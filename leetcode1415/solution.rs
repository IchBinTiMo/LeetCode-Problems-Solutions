/*
Solution: DFS

Time: O(3 ^ n) | Space: O(n)

Runtime: 0 ms | 100%
Memory: 2.33 MB | 37.50%
*/

impl Solution {
    pub fn get_happy_string(n: i32, mut k: i32) -> String {
        let mut res: String = String::new();

        Self::dfs(&mut res,n, &mut k);

        res
    }
    fn dfs(res: &mut String, n: i32, k: &mut i32) {
        if n == 0 {
            *k -= 1;
            return;
        } else {
            for c in ['a', 'b', 'c'] {
                if res.chars().last() == Some(c) {
                    continue;
                } else {
                    res.push(c);

                    Self::dfs(res, n - 1, k);

                    if *k == 0 {
                        return;
                    }

                    res.pop();

                }
            }
        }
    }
}

/*
Solution: Backtracking

Time: O(3 ^ n) | Space: O(n)

Runtime: 29 ms | -%
Memory: 2.22 MB | 87.50%
*/

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut res: String = String::new();
        let mut path: String = String::new();
        let mut current: i32 = 0;

        Self::backtrack(&mut res, &mut path, &mut current, '0', n as usize, k);

        res
    }
    fn backtrack(res: &mut String, path: &mut String, current: &mut i32, prev: char, n: usize, k: i32) {
        if (*path).len() == n {
            *current += 1;
            if *current == k {
                *res = path.clone();
            }
            return;
        } else if (*path).len() == n {
            return;
        } else {
            for c in ['a', 'b', 'c'] {
                if c == prev {
                    continue;
                }
                (*path).push(c);
                println!("{path:?}");
                Self::backtrack(res, path, current, c, n, k);
                (*path).pop();

                if !(*res).is_empty() {
                    return;
                }
            }
        }
    }
}
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
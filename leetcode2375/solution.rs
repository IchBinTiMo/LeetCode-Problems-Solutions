/*
Solution: backtrack

Time: O(9!) | Space: O(1)

Runtime: 0 ms | 100.00%
Memory: 2.22 MB | 33.33%
*/

impl Solution {
    pub fn smallest_number(pattern: String) -> String {
        let mut visited: [bool; 10] = [false; 10];

        let pattern: Vec<char> = pattern.chars().collect::<Vec<char>>();

        let mut path: usize = 0;

        let mut res: String = String::new();

        Self::backtrack(&mut res, &mut path, 1, &mut visited, &pattern);

        res
    }

    fn backtrack(res: &mut String, path: &mut usize, current: usize, visited: &mut [bool; 10], pattern: &Vec<char>) {
        if current - 2 == pattern.len() {
            let tmp: String = format!("{path}");
            if *res > tmp || (*res).is_empty() {
                *res = tmp;
            }
            return;
        } else {
            for i in 1..10 {
                if !(*res).is_empty() {
                    return;
                }
                if !visited[i] {
                    if *path == 0 || 
                        (pattern[current - 2] == 'I' && *path % 10 < i) ||
                        (pattern[current - 2] == 'D' && *path % 10 > i) {
                            *path = *path * 10 + i;
                            visited[i] = true;

                            Self::backtrack(res, path, current + 1, visited, pattern);

                            *path /= 10;
                            visited[i] = false;
                    }
                }
            }
        }
    }
}
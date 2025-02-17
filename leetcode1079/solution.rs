/*
Solution: Backtrack

Time: O(n!) | Space: O(n)

Runtime: 9 ms | 46.15%
Memory: 2.91 MB | 46.15%

- n: length of 'tiles'
*/

use std::collections::HashSet;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {

        let mut freqs: [i32; 26] = [0; 26];

        for b in tiles.bytes() {
            freqs[(b - b'A') as usize] += 1;
        }

        let mut indices: Vec<usize> = Vec::new();

        for i in 0..26 {
            if freqs[i] > 0 {
                indices.push(i);
            }
        }

        let mut set: HashSet<String> = HashSet::new();
        let mut path: String = String::new();
        
        Self::backtrack(&mut set, &mut path, &indices, &mut freqs);

        set.len() as i32 - 1
    }
    
    fn backtrack(set: &mut HashSet<String>, path: &mut String, indices: &Vec<usize>, freqs: &mut [i32; 26]) {
        if set.contains(path) {
            return;
        } else {
            set.insert(path.to_string());
            for i in 0..indices.len() {
                if freqs[indices[i]] > 0 {
                    let c: char = ((indices[i] as u8) + b'A') as char;

                    freqs[indices[i]] -= 1;

                    path.push(c);

                    Self::backtrack(set, path, indices, freqs);

                    path.pop();

                    freqs[indices[i]] += 1;
                }
            }
        }
    }
}
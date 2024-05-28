impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        /// DP with optimization
        /// 
        /// Time O(n^2) | Space O(n)
        /// where n is the length of grid
        let n: usize = grid.len();

        let mut prev: Vec<i32> = grid[0].clone();

        for i in 1..n {
            let mut current: Vec<i32> = vec![i32::MAX; n];

            let (min, min2) = Self::get_min(&prev);


            for j in 0..n {
                if prev[j] == min {
                    current[j] = grid[i][j] + min2;
                } else {
                    current[j] = grid[i][j] + min;
                }
            }

            prev = current;
        }

        *prev.iter().min().unwrap()
    }

    fn get_min(row: &[i32]) -> (i32, i32) {
        let (mut min, mut min2): (i32, i32) = (i32::MAX, i32::MAX);

        for i in 0..row.len() {
            if row[i] < min {
                min2 = min;
                min = row[i];
            } else if row[i] < min2 {
                    min2 = row[i];
            }
        }

        (min, min2)
    }
}

// impl Solution {
//     pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
//         /// DP without optimization
//         /// 
//         /// Time O(n^3) | Space O(n)
//         /// where n is the length of grid
//         let n: usize = grid.len();

//         let mut prev: Vec<i32> = grid[0].clone();

//         for i in 1..n {
//             let mut current: Vec<i32> = vec![i32::MAX; n];

//             for j in 0..n {
//                 for k in 0..n {
//                     if j != k {
//                         current[j] = current[j].min(prev[k] + grid[i][j]);
//                     }
//                 }
//             }

//             prev = current;
//         }

//         *prev.iter().min().unwrap()
//     }
// }
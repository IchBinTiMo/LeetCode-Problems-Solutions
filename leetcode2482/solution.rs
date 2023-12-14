impl Solution {
    pub fn ones_minus_zeros(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();

        let mut dRows: Vec<i32> = vec![0; m];
        let mut dCols: Vec<i32> = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                dRows[i] += grid[i][j];
                dCols[j] += grid[i][j];
            }
        }

        let mut ans: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                grid[i][j] = 2 * dRows[i] + 2 * dCols[j] - m as i32 - n as i32;
            }
        }

        grid
    }
}

// impl Solution {
//     pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
//         let m = grid.len();
//         let n = grid[0].len();

//         let mut dRows: Vec<i32> = vec![0; m];
//         let mut dCols: Vec<i32> = vec![0; n];

//         for i in 0..m {
//             for j in 0..n {
//                 match grid[i][j] {
//                     1 => {
//                         dRows[i] += 1;
//                         dCols[j] += 1;
//                     },
//                     _ => {
//                         dRows[i] -= 1;
//                         dCols[j] -= 1;
//                     }
//                 }
//             }
//         }

//         let mut ans: Vec<Vec<i32>> = vec![vec![0; n]; m];

//         for i in 0..m {
//             for j in 0..n {
//                 ans[i][j] = dRows[i] + dCols[j];
//             }
//         }

//         ans
//     }
// }

// // impl Solution {
// //     pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
// //         let m = grid.len();
// //         let n = grid[0].len();

// //         let mut onesRows: Vec<i32> = vec![0; m];
// //         let mut zeroesRows: Vec<i32> = vec![0; m];
// //         let mut onesCols: Vec<i32> = vec![0; n];
// //         let mut zeroesCols: Vec<i32> = vec![0; n];

// //         for i in 0..m {
// //             for j in 0..n {
// //                 match grid[i][j] {
// //                     1 => {
// //                         onesRows[i] += 1;
// //                         onesCols[j] += 1;
// //                     },
// //                     _ => {
// //                         zeroesRows[i] += 1;
// //                         zeroesCols[j] += 1;
// //                     }
// //                 }
// //             }
// //         }

// //         let mut ans: Vec<Vec<i32>> = vec![vec![0; n]; m];

// //         for i in 0..m {
// //             for j in 0..n {
// //                 ans[i][j] = onesRows[i] + onesCols[j] - zeroesRows[i] - zeroesCols[j];
// //             }
// //         }

// //         ans
// //     }
// // }
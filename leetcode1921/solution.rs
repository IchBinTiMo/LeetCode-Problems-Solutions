impl Solution {
    pub fn eliminate_maximum(mut dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut monsters: Vec<i32> = Vec::new();

        for (d, s) in dist.iter().zip(speed.iter()) {
            monsters.push((*d as f64 / *s as f64).ceil() as i32 );
        }

        monsters.sort_unstable();
        
        for &m in monsters.iter() {
            if ans >= m {
                break;
            }
            ans += 1;
        }

        ans 
    }
}

// impl Solution {
//     pub fn eliminate_maximum(mut dist: Vec<i32>, speed: Vec<i32>) -> i32 {
//         let mut ans = 0;

//         let mut monsters: Vec<i32> = Vec::new();

//         for (d, s) in dist.iter().zip(speed.iter()) {
//             monsters.push((*d as f64 / *s as f64).ceil() as i32 );
//         }

//         monsters.sort();
        
//         for &m in monsters.iter() {
//             if ans >= m {
//                 break;
//             }
//             ans += 1;
//         }
//         ans 
//     }
// }

// // impl Solution {
// //     pub fn eliminate_maximum(mut dist: Vec<i32>, speed: Vec<i32>) -> i32 {
// //         let mut ans = 1;

// //         let mut monsters: Vec<i32> = Vec::new();

// //         for (m, s) in dist.iter().zip(speed.iter()) {
// //             monsters.push((*m as f64 / *s as f64).ceil() as i32 );
// //         }

// //         monsters.sort();
// //         monsters.remove(0);

// //         while monsters.len() > 0 && monsters[0] - ans > 0 {
// //             ans += 1;
// //             monsters.remove(0);
// //         }

// //         ans 
// //     }
// // }
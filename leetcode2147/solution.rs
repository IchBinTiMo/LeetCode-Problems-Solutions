impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {

        let mut ans: i64 = 1;

        let mut count = 0;

        let mut last_seat: usize = 0;

        let mut total_seats = 0;

        for (i, cor) in corridor.chars().enumerate() {
            match cor {
                'S' => {
                    total_seats += 1;
                    count += 1;
                    if count == 2 {
                        count = 0;
                        last_seat = i;
                    } else if count == 1 && last_seat != 0 {
                        ans = ans * ((i - last_seat) as i64) % 1_000_000_007;
                    }
                },
                _ => continue
            }
        }

        if count == 1 {
            return 0;
        }

        if total_seats == 0 {
            return 0;
        }

        ans as i32
    }
}
// impl Solution {
//     pub fn number_of_ways(corridor: String) -> i32 {
//         let mut count = 0;

//         let mut tmp: Vec<i64> = vec![];

//         let mut last_seat: usize = 0;

//         let mut total_seats = 0;

//         for (i, cor) in corridor.chars().enumerate() {
//             match cor {
//                 'S' => {
//                     total_seats += 1;
//                     count += 1;
//                     if count == 2 {
//                         count = 0;
//                         last_seat = i;
//                     } else if count == 1 {
//                         tmp.push((i - last_seat) as i64);
//                     }
//                 },
//                 _ => continue
//             }
//         }

//         if count == 1 {
//             return 0;
//         }

//         if total_seats == 0 {
//             return 0;
//         }

//         tmp.remove(0);


//         (tmp.iter().fold(1, |acc, val| acc * val % 1_000_000_007)) as i32
//     }
// }
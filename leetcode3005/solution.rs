impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freqs: [i32; 100] = [0; 100];

        for num in nums {
            freqs[(num as usize) - 1] += 1;
        }

        let mut res: i32 = 0;
        let mut max_freq: i32 = i32::MIN;

        for freq in freqs {
            if freq > max_freq {
                max_freq = freq;
                res = max_freq;
            } else if freq == max_freq {
                res += freq;
            }
        }

        res
    }
}

// impl Solution {
//     pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
//         let mut freqs: [i32; 100] = [0; 100];

//         for num in nums {
//             freqs[(num as usize) - 1] += 1;
//         }

//         let mut res: i32 = 0;
//         let mut max_freq: i32 = i32::MIN;

//         for freq in freqs {
//             if freq > max_freq {
//                 max_freq = freq;
//                 res = max_freq;
//             } else if freq == max_freq {
//                 res += freq;
//             } else {
//                 continue;
//             }
//         }

//         res
//     }
// }
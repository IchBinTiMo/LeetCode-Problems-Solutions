impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        /// Time O(m + n) | Space O(m + n)
        /// where m is the length of arr1
        /// and n is the length of arr2
        let mut freqs: [i32; 1001] = [0; 1001];

        // count the frequency of each number in arr1
        for &num in arr1.iter() {
            freqs[num as usize] += 1;
        }

        let mut res: Vec<i32> = Vec::new();

        // push number to `res` as they are in the same order as in `arr2`
        // and reset their frequencies to 0
        for &num in arr2.iter() {
            for _ in 0..freqs[num as usize] {
                res.push(num);
            }

            freqs[num as usize] = 0;
        }

        // push the remained numbers to `res`
        for i in 0..freqs.len() {
            for _ in 0..freqs[i] {
                res.push(i as i32);
            }
        }

        res
    }
}

// impl Solution {
//     pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
//         /// Time O(mlogm) | Space O(m + n)
//         /// where m is the length of arr1
//         /// and n is the length of arr2
//         let mut indices: Vec<usize> = vec![1002; 1001];

//         // store the indices of numbers in arr2
//         for i in 0..arr2.len() {
//             indices[arr2[i] as usize] = i;
//         }

//         let mut remained: Vec<i32> = Vec::new();

//         let mut freqs: Vec<i32> = vec![0; 1001];

//         // count the frequency of each number in arr1 if it is in arr2
//         // if not in arr2, push it to `remained`
//         for &num in arr1.iter() {
//             if indices[num as usize] != 1002 {
//                 freqs[indices[num as usize]] += 1;
//             } else {
//                 remained.push(num);
//             }
//         }

//         let mut res: Vec<i32> = Vec::new();

//         // push number to `res` as they are in the same order as in `arr2`
//         for i in 0..freqs.len() {
//             if freqs[i] == 0 {
//                 break;
//             }
//             for _ in 0..freqs[i] {
//                 res.push(arr2[i]);
//             }
//         }

//         // this is why the time complexity is O(mlogm)
//         remained.sort_unstable();

//         // append `remained` to `res`
//         res.append(&mut remained);

//         res
//     }
// }
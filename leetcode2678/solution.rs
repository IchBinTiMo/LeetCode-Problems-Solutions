/*
Solution 1:

Time: O(n) | Space: O(1)

-n: length of details
*/

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut res: i32 = 0;

        for d in details.iter() {
            let d: Vec<u8> = d.bytes().collect::<Vec<u8>>();
            let age: u8 = (d[11] - 48) * 10 + d[12] - 48;

            res += if (age > 60) {
                1
            } else {
                0
            };
        }

        res
    }
}
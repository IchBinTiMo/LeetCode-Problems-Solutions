impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        /// Time: O(n log n) | Space: O(1)
        /// where n is the length of seats
        seats.sort_unstable();
        students.sort_unstable();

        let mut res: i32 = 0;

        for (&seat, &student) in seats.iter().zip(students.iter()) {
            res += (seat - student).abs();
        }

        res
    }
}
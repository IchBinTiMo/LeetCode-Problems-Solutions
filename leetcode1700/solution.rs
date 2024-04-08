impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        // the top of stack "sandwiches"
        let mut top: usize = 0;

        // the count of how many students prefer for each type
        let mut count: [i32; 2] = [0; 2];

        for &student in students.iter() {
            count[student as usize] += 1;
        }

        // loop if there is at least one student prefers the type of sandwich that is on the top of the stack, otherwise stop
        while top < students.len() && count[sandwiches[top] as usize] != 0 {
            count[sandwiches[top] as usize] -= 1;
            top += 1;
        }

        count[0] + count[1]
    }
}
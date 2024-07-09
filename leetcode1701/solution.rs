impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        /// Time O(N) | Space O(1)
        /// 
        /// where N is the length of customers
        let mut acc: f64 = 0.0;
        let mut current: i32 = 0;

        for c in customers.iter() {
            if current < c[0] {
                current = c[0] + c[1];
                acc += c[1] as f64;
            } else {
                current += c[1];
                acc += (current - c[0]) as f64;
            }
        }

        acc / (customers.len() as f64)
    }
}
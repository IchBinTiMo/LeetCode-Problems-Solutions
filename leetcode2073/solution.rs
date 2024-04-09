impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k: usize = k as usize;

        let mut res: i32 = 0;

        for i in 0..tickets.len() {
            res += tickets[i].min(tickets[k] - ((i > k) as i32));
        }

        res
    }
}
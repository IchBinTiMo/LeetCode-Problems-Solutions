use std::collections::VecDeque;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n as usize];

        for flight in flights {
            let from: usize = flight[0] as usize;
            let to: usize = flight[1] as usize;
            let price: i32 = flight[2];

            graph[from].push((to, price));
        }

        let mut stack: VecDeque<(usize, i32, i32)> = VecDeque::new();
        let mut dists: Vec<i32> = vec![i32::MAX; n as usize];

        stack.push_back((src as usize, 0, -1));

        while let Some((from, total_price, stop)) = stack.pop_front() {
            if stop >= k {
                break;
            }

            for &(to, price) in &graph[from] {
                if total_price + price < dists[to] {
                    dists[to] = total_price + price;
                    stack.push_back((to as usize, total_price + price, stop + 1));
                }
            }
        }

        return if dists[dst as usize] == i32::MAX {-1} else {dists[dst as usize]}
    }
}

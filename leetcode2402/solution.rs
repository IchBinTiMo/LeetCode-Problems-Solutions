use std::collections::VecDeque;
use std::iter::FromIterator;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        meetings.sort_unstable_by_key(|k| k[0]);
        let mut meeting_queue: VecDeque<Vec<i32>> = VecDeque::from_iter(meetings);

        let mut free: BinaryHeap<Reverse<(usize, i64)>> = BinaryHeap::with_capacity(n);
        let mut occupied: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        let mut counts: Vec<i32> = vec![0; n];

        for i in 0..n {
            free.push(Reverse((i, 0)));
        }

        while let Some(meeting) = meeting_queue.pop_front() {
            while !occupied.is_empty() && occupied.peek().unwrap().0.0 <= meeting[0] as i64 {
                if let Some(Reverse((end, idx))) = occupied.pop() {
                    free.push(Reverse((idx, end)));
                }
            }

            if free.is_empty() {
                if let Some(Reverse((end, idx))) = occupied.pop() {
                    free.push(Reverse((idx, end)));
                }
            }
            if let Some(Reverse((idx, end))) = free.pop() {
                counts[idx] += 1;
                occupied.push(Reverse((end.max(meeting[0] as i64) + (meeting[1] - meeting[0]) as i64, idx)));
            }
        }

        let mut ans: usize = 0;

        for i in 0..n {
            if counts[i] > counts[ans] {
                ans = i;
            } 
        }

        ans as i32
    }
}
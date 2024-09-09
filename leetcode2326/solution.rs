// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

/*
Solution:

Time: O(k) | Space: O(m * n)

Runtime: 85 ms | 88.00%
Memory: 10.68 MB | 68.00%

- k: number of nodes in linked list 'head'
*/
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {

        let mut current = head;
        let mut r: i32 = 0;
        let mut c: i32 = 0;

        let mut direction: usize = 0;
        let steps: [i32; 5] = [0, 1, 0, -1, 0];

        let mut res: Vec<Vec<i32>> = vec![vec![-1; n as usize]; m as usize];

        while let Some(node) = current.take() {
            res[r as usize][c as usize] = node.val;
            current = node.next;

            let new_r: i32 = r + steps[direction];
            let new_c: i32 = c + steps[direction + 1];

            if new_r >= m || new_r < 0 ||
                new_c >= n || new_c < 0 ||
                res[new_r as usize][new_c as usize] != -1 {
                    direction = (direction + 1) % 4;
                }

            r = (r + steps[direction] + m) % m;
            c = (c + steps[direction + 1] + n) % n;
        }

        res
    }
}
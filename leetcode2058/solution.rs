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
impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        /// Time: O(n) | Space: O(1)
        /// 
        /// where n is the number of nodes in the linked list
        let mut first: i32 = -1;
        let mut prev_point: i32 = -1;
        let mut mini: i32 = i32::MAX;
        let mut idx: i32 = 1;

        let tmp = head.unwrap();

        let mut prev = tmp.val;
        let mut current = tmp.next.unwrap();

        while let Some(node) = current.next {
            if (current.val > prev && current.val > node.val) || (current.val < prev && current.val < node.val) {
                if first == -1 {
                    first = idx;
                } else {
                    mini = mini.min(idx - prev_point);
                }
                prev_point = idx;
            }

            prev = current.val;
            current = node;
            idx += 1;
        }

        Vec::from(
            if mini == i32::MAX {
                [-1, -1]
            } else {
                [mini, prev_point - first]
            })
    }
}
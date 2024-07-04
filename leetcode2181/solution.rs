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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        /// Time: O(n) | Space: O(1)
        /// 
        /// where n is the number of nodes in the list
        let mut res: Option<Box<ListNode>> = None;
        let mut left: &mut Option<Box<ListNode>> = &mut res;
        let mut right: Option<Box<ListNode>> = head;

        while let Some(mut node) = right {
            let mut sum = node.val;

            loop {
                node = node.next.unwrap();

                if node.val == 0 {
                    right = node.next.take();
                    node.val = sum;
                    left = &mut left.insert(node).next;
                    break;
                }

                sum += node.val;
            }
        }

        res
        
    }
}
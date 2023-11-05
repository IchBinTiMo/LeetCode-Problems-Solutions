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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current = head.clone();
        Solution::reverse_in_pairs(&mut current);

        current
    }

    pub fn reverse_in_pairs(head: &mut Option<Box<ListNode>>) {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return;
        }

        let mut next = head.as_mut().unwrap().next.take();
        Solution::reverse_in_pairs(&mut next.as_mut().unwrap().next);
        head.as_mut().unwrap().next = next.as_mut().unwrap().next.take();
        next.as_mut().unwrap().next = head.take();
        *head = next;
    }
}
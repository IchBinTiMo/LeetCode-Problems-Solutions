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
    pub fn merge_in_between(mut list1: Option<Box<ListNode>>, a: i32, b: i32, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current: &mut Option<Box<ListNode>> = &mut list1;

        // find the node before the first node to be replaced
        for _ in 0..(a - 1) {
            current = &mut current.as_mut().unwrap().next;
        }

        // replace the first node and get the node replaced
        let mut remained: Option<Box<ListNode>> = std::mem::replace(&mut current.as_mut().unwrap().next, list2);

        // find the last node
        while current.as_mut().unwrap().next.is_some() {
            current = &mut current.as_mut().unwrap().next;
        }

        // find the node after the last node to be replaced
        for _ in 0..=(b - a) {
            remained = remained.unwrap().next;
        }

        // replace the last node
        current.as_mut().unwrap().next = remained;

        list1
    }
}
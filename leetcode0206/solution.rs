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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current: &mut Option<Box<ListNode>> = &mut head;
        let mut prev: Option<Box<ListNode>> = None;

        while current.is_some() {
            // swap
            let mut tmp: Option<Box<ListNode>> = std::mem::replace(&mut current.as_mut().unwrap().next, prev);
            prev = (*current).take();
            *current = tmp;
        }

        prev
    }
}

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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut current: &mut Option<Box<ListNode>> = &mut head;
        let mut prev: Option<Box<ListNode>> = None;

        while current.is_some() {
            // swap
            let mut tmp: Option<Box<ListNode>> = std::mem::replace(&mut current.as_mut().unwrap().next, prev);
            prev = (*current).clone();
            *current = tmp;
        }

        prev
    }
}
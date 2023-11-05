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
use std::collections::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();

        for list in lists {
            let mut current = list;
            while let Some(node) = current {
                heap.push(node.val);
                current = node.next;
            }
        }
        
        if heap.peek().is_some() {
            let mut ans = Box::new(ListNode::new(heap.pop().unwrap()));
            while let Some(val) = heap.pop() {
                let mut tmp = Box::new(ListNode::new(val));
                tmp.next = Some(ans);
                ans = tmp;
            }
            return Some(ans);
        }
        
        None
    }
}

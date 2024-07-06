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
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        /// Time: O(n) | Space: O(n)
        /// 
        /// n = max(len(l1), len(l2))
        let mut res: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut current = res.as_mut().unwrap();
        let mut flag: i32 = 0;

        loop {
            let mut val: i32 = flag;

            if let Some(node) = l1 {
                val += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                val += node.val;
                l2 = node.next;
            }

            flag = val / 10;
            val %= 10;

            current.val = val;

            if l1 == None && l2 == None && flag == 0 {
                break;
            }

            current.next = Some(Box::new(ListNode::new(0)));
            current = current.next.as_mut().unwrap();



        }

        res
    }
}
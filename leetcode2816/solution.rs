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
    pub fn double_it(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut stack: Vec<Box<ListNode>> = Vec::new();
        let mut current : Option<Box<ListNode>> = head;

        while let Some(mut node) = current.take() {
            current = node.next.take();
            stack.push(node);
        }

        let mut flag: i32 = 0;
        let mut next: Option<Box<ListNode>> = None;

        while let Some(mut node) = stack.pop() {
            node.val *= 2;
            node.val += flag;
            flag = node.val / 10;
            node.val = node.val % 10;
            

            node.next = next.take();
            next = Some(node);
        }

        if flag == 1 {
            next = Some(Box::new(ListNode{val: 1, next}));
        }

        next
    }
}
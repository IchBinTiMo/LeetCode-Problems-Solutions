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

use std::collections::HashSet;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sums: Vec<i32> = Vec::new();

        let mut set: HashSet<i32> = HashSet::new();

        let mut current_sum: i32 = 0;

        let mut current_node: &Option<Box<ListNode>> = &head;

        sums.push(0);

        set.insert(0);

        while let Some(node) = current_node {
            current_sum += node.val;
            if let Some(_) = set.get(&current_sum) {
                while let Some(&sum) = sums.last() {
                    sums.pop();
                    set.remove(&sum);

                    if sum == current_sum {
                        break;
                    }
                }
            }
            sums.push(current_sum);
            set.insert(current_sum);

            current_node = &node.next;
        }


        let mut res_head: Option<Box<ListNode>> = None;

        while let Some(sum) = sums.pop() {
            if let Some(&next_sum) = sums.last() {
                let next = res_head.take();
                res_head = Some(Box::new(ListNode{val: sum - next_sum, next}));
            }
        }

        res_head
    }
}

// // Definition for singly-linked list.
// // #[derive(PartialEq, Eq, Clone, Debug)]
// // pub struct ListNode {
// //   pub val: i32,
// //   pub next: Option<Box<ListNode>>
// // }
// // 
// // impl ListNode {
// //   #[inline]
// //   fn new(val: i32) -> Self {
// //     ListNode {
// //       next: None,
// //       val
// //     }
// //   }
// // }

// use std::collections::HashSet;

// impl Solution {
//     pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//         let mut sums: Vec<i32> = Vec::new();

//         let mut set: HashSet<i32> = HashSet::new();

//         let mut current_sum: i32 = 0;

//         let mut current_node: Option<Box<ListNode>> = head.clone();

//         sums.push(0);

//         set.insert(0);

//         while let Some(ref node) = current_node {
//             current_sum += node.val;
//             if let Some(_) = set.get(&current_sum) {
//                 while let Some(&sum) = sums.last() {
//                     sums.pop();
//                     set.remove(&sum);

//                     if sum == current_sum {
//                         break;
//                     }
//                 }
//             }
//             sums.push(current_sum);
//             set.insert(current_sum);

//             current_node = node.next.clone();
//         }


//         let mut res_head: Option<Box<ListNode>> = None;

//         while let Some(sum) = sums.pop() {
//             if let Some(&next_sum) = sums.last() {
//                 let next = res_head.take();
//                 res_head = Some(Box::new(ListNode{val: sum - next_sum, next}));
//             }
//         }

//         res_head
//     }
// }
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::collections::VecDeque;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // a queue to traverse the tree
        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        // an array to store the left nodes
        let mut left: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        let mut res: i32 = 0;

        queue.push_back(root);

        while let Some(n) = queue.pop_front() {
            if let Some(node) = n {
                let node = node.borrow();

                // store the left child of the current node
                left.push_back(node.left.clone());

                // store the node to be processed in the next iteration
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());

            }
        }

        // traverse the left nodes and sum up the values if they are leaves
        while let Some(n) = left.pop_front() {
            if let Some(node) = n {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    res += node.val;
                }
            }
        }

        res
    }
}
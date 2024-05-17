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
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        /// DFS
        /// 
        /// Time: O(n) | Space: O(n)
        if let Some(node) = root {
            let mut nd = node.borrow_mut();

            if nd.left.is_some() {
                nd.left = Self::remove_leaf_nodes(nd.left.take(), target);
            }

            if nd.right.is_some() {
                nd.right = Self::remove_leaf_nodes(nd.right.take(), target);
            }

            if nd.left.is_none() && nd.right.is_none() {
                if nd.val == target {
                    return None;
                }
            }

            drop(nd);

            return Some(node);
        } else {
            None
        }
    }
}
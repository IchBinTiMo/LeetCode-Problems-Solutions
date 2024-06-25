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
    /// DFS
    /// 
    /// Time: O(n) | Space: O(n)
    /// where n is the number of nodes in the tree
    pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut acc: i32 = 0;

        Self::traverse(&mut acc, root)
    }

    // In-order traversal and traverse right first instead of left
    fn traverse(acc: &mut i32, root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref node) = root {
            let mut node = node.borrow_mut();
            Self::traverse(acc, node.right.clone()); // traverse right first
            node.val += *acc; // update the node value
            *acc = node.val; // update the accumulator
            Self::traverse(acc, node.left.clone()); // traverse left last
        }

        root
    }
}
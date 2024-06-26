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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        /// in-order traversal
        /// 
        /// Time: O(N) | Space: O(N)
        /// where N is the number of nodes in the tree
        let mut v: Vec<i32> = Vec::new();

        Self::in_order(&root, &mut v);

        Self::build_tree(&v, 0, v.len())
    }

    fn in_order(root: &Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
        if let Some(node) = root {
            let node = node.borrow();
            Self::in_order(&node.left, v);
            v.push(node.val);
            Self::in_order(&node.right, v);
        }
    }

    fn build_tree(v: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if left == right {
            return None;
        }

        let mid: usize = (left + right) / 2;

        Some(Rc::new(RefCell::new(TreeNode {
            val: v[mid],
            left: Self::build_tree(v, left, mid),
            right: Self::build_tree(v, mid + 1, right),
        })))
    }
}
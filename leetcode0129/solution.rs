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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        /// DFS
        Self::dfs(root.unwrap(), 0)
    }

    fn dfs(root: Rc<RefCell<TreeNode>>, mut path: i32) -> i32 {
        let mut node = root.borrow_mut();

        path += node.val;

        match (node.left.take(), node.right.take()) {
            // if current node is not a leaf node then multiply current value by 10 and traverse its children
            (Some(left), Some(right)) => Self::dfs(left, path * 10) + Self::dfs(right, path * 10),
            (Some(left), None) => Self::dfs(left, path * 10),
            (None, Some(right)) => Self::dfs(right, path * 10),
            // if current node is a leaf node then return current value
            (None, None) => path,
        }
    }
}
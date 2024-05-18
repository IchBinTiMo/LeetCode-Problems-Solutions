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
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        /// DFS
        /// 
        /// Time O(N) | Space O(H) where N is the number of nodes in the tree and H is the height of the tree
        let mut res: i32 = 0;

        Self::dfs(&mut res, &root);
        
        res
    }

    fn dfs(res: &mut i32, root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();

            // negative values mean insufficient coins
            // while positive values mean sufficient or exceeded coins
            // in the left and right subtrees
            let left = Self::dfs(res, &node.left);
            let right = Self::dfs(res, &node.right);

            // if the value is negative, it takes one step to input one coin
            // if the value is positive, it takes one step to output one coin
            // so we add the absolute value to the result
            *res += left.abs() + right.abs();

            // -1 means we need to leave one coin in the current node
            node.val + left + right - 1
        } else {
            0
        }
    }
}
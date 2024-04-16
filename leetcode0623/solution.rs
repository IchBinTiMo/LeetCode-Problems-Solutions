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
    pub fn add_one_row(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        /// DFS
        
        if depth == 1 { // corner case
            Some(Rc::new(RefCell::new(TreeNode {
                val: val,
                left: root,
                right: None
            })))
        } else {
            Self::dfs(root, val, 1, depth)
        }
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, val: i32, current_depth: i32, target_depth: i32) -> Option<Rc<RefCell<TreeNode>>> {
        if current_depth == target_depth - 1 {
            if let Some(root_node) = root {
                let mut node = root_node.borrow_mut();

                Some(Rc::new(RefCell::new(TreeNode {
                    val: node.val,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                            val: val,
                            left: node.left.take(),
                            right: None
                        }))),
                    right:  Some(Rc::new(RefCell::new(TreeNode {
                                val: val,
                                left: None,
                                right: node.right.take()
                            })))
                })))

            } else {
                None
            }
        } else {
            if let Some(root_node) = root {
                let mut node = root_node.borrow_mut();

                Some(Rc::new(RefCell::new(TreeNode {
                    val: node.val,
                    left: Self::dfs(node.left.take(), val, current_depth + 1, target_depth),
                    right: Self::dfs(node.right.take(), val, current_depth + 1, target_depth)
                })))

            } else {
                None
            }


        }
    }
}
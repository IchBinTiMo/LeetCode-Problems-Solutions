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
use std::collections::VecDeque;
impl Solution {
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        /// DFS
        let mut res: VecDeque<u8> = VecDeque::new();

        Self::dfs(&mut res, root.unwrap(), VecDeque::new());

        res.into_iter().map(|x| x as char).collect()
    }

    fn dfs(res: &mut VecDeque<u8>, root: Rc<RefCell<TreeNode>>, mut path: VecDeque<u8>) {

        let mut node = root.borrow_mut();

        path.push_front((node.val + 97) as u8);

        match (node.left.take(), node.right.take()) {
            (Some(left), Some(right)) => {
                Self::dfs(res, left, path.clone());
                Self::dfs(res, right, path.clone());
            },
            (Some(left), None) => {
                Self::dfs(res, left, path.clone());
                path.pop_front();
                },
            (None, Some(right)) => {
                Self::dfs(res, right, path.clone());
                path.pop_front();
                },
            (None, None) => {
                if *res > path || res.is_empty() {
                    *res = path;
                }
            }
        }

    }
}
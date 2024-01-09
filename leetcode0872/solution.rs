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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut leaves1: Vec<i32> = Vec::new();
        let mut leaves2: Vec<i32> = Vec::new();

        Solution::dfs(&mut leaves1, root1);
        Solution::dfs(&mut leaves2, root2);


        leaves1 == leaves2
    }

    pub fn dfs(leaves: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(rt) = root {
            let r = rt.borrow();
            if r.left.is_none() && r.right.is_none() {
                leaves.push(r.val);
                return;
            }
            Solution::dfs(leaves, r.left.clone());
            Solution::dfs(leaves, r.right.clone());
        }
    }
}
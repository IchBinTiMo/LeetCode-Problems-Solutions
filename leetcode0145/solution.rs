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

/*
Solution: Postorder Traversal

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.01 MB | 84.51%

- n: number of nodes in the tree
*/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        Self::traverse(&mut res, root);

        res        
    }

    fn traverse(res: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
        if (root.is_none()) {
            return;
        }
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            Self::traverse(res, node.left.take());
            Self::traverse(res, node.right.take());
            res.push(node.val);
        }
    }
}
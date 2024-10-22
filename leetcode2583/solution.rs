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
Solution: DFS

Time: O(n log n) | Space: O(n)

Runtime: 13 ms | 100.00%
Memory: 19.21 MB | 33.33%

- n: number of nodes in the tree
*/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut sums: Vec<i64> = Vec::new();

        Self::traverse(&mut sums, 0, root);

        sums.sort_unstable();

        if (k as usize) > sums.len() {
            return -1;
        }

        sums[sums.len() - k as usize]
    }

    fn traverse(sums: &mut Vec<i64>, current: usize, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            if current >= sums.len() {
                sums.push(node.val as i64);
            } else {
                sums[current] += node.val as i64;
            }

            Self::traverse(sums, current + 1, node.left.take());
            Self::traverse(sums, current + 1, node.right.take());
        }
    }
}
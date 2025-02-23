/*
Solution: DFS

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.27 MB | 40.00%

- n: length of 'preorder'
*/

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

use std::collections::HashMap;
impl Solution {
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n: usize = preorder.len();

        let mut map: HashMap<i32, (usize, usize)> = HashMap::new();
        let mut children: HashMap<i32, (Option<i32>, Option<i32>)> = HashMap::new();

        for k in 0..n {
            map.entry(preorder[k]).and_modify(|(i, _)| *i = k).or_insert((k, 0));
            map.entry(postorder[k]).and_modify(|(_, j)| *j = k).or_insert((0, k));

        }

        for i in 0..(n - 1) {
            let num: i32 = preorder[i];

            if let Some(&(_, j)) = map.get(&num) {
                if i < n && j >= 1 {
                    if preorder[i + 1] != postorder[j - 1] {
                        let (pre, post): (usize, usize) = *map.get(&preorder[i + 1]).unwrap();

                        if pre < i || post > j {
                            continue;
                        }

                        let (pre, post): (usize, usize) = *map.get(&postorder[j - 1]).unwrap();

                        if pre < i || post > j {
                            continue;
                        }

                        children.insert(num, (Some(preorder[i + 1]), Some(postorder[j - 1])));
                    } else if preorder[i + 1] == postorder[j - 1] {
                        children.insert(num, (Some(preorder[i + 1]), None));
                    }
                }
            }
        }

        Self::dfs(preorder[0], &children)
    }

    fn dfs(root: i32, children: &HashMap<i32, (Option<i32>, Option<i32>)>) -> Option<Rc<RefCell<TreeNode>>> {

        let mut node: TreeNode = TreeNode::new(root);

        if let Some(&(left, right)) = children.get(&root) {
            if let Some(left) = left {
                node.left = Self::dfs(left, children);
            }

            if let Some(right) = right {
                node.right = Self::dfs(right, children);
            }
        }

        Some(Rc::new(RefCell::new(node)))
    }
}
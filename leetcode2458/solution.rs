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

Time: O(n + q) | Space: O(n)

Runtime: 39 ms | 100.00%
Memory: 18.95 MB | 50.00%

- n: number of nodes
- q: number of queries
*/
use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut depths: HashMap<i32, i32> = HashMap::new();
        let mut orders: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        let max_height: i32 = Self::traverse(0, &mut depths, &mut orders, root);

        let mut res: Vec<i32> = Vec::new();

        for &q in queries.iter() {
            let depth: i32 = *depths.get(&q).unwrap();

            let order: &Vec<(i32, i32)> = orders.get(&depth).unwrap();

            if order[0].1 == q {
                res.push(max_height - order[0].0 + order[1].0 - 1);
            } else {
                res.push(max_height - 1);
            }
        }

        res
    }

    fn traverse(depth: i32, depths: &mut HashMap<i32, i32>, orders: &mut HashMap<i32, Vec<(i32, i32)>>, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            depths.insert(node.val, depth);

            let left: i32 = Self::traverse(depth + 1, depths, orders, node.left.take());
            let right: i32 = Self::traverse(depth + 1, depths, orders, node.right.take());

            let height: i32 = left.max(right) + 1;

            orders.entry(depth).and_modify(|order| {
                if height > order[0].0 {
                    order[1] = order[0];
                    order[0] = (height, node.val);
                } else if height > order[1].0 {
                    order[1] = (height, node.val);
                }
            }).or_insert(vec![(height, node.val), (0, 0)]);

            return height;
        } else {
            return 0;
        }
    }
}
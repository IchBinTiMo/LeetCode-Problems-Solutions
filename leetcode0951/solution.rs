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

Time: O(n) | Space: O(n)

Runtime: 0 ms | 100.00%
Memory: 2.3 MB | 100.00%

- n: sum of number of nodes in root1 and root2
*/

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut map1: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut map2: HashMap<i32, HashSet<i32>> = HashMap::new();

        Self::traverse(&mut map1, -1, root1);
        Self::traverse(&mut map2, -1, root2);

        map1 == map2
    }

    fn traverse(map: &mut HashMap<i32, HashSet<i32>>, parent: i32, root: Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node = node.borrow_mut();

            map.entry(parent).and_modify(|set| {set.insert(node.val);}).or_insert(HashSet::from([node.val]));
            
            Self::traverse(map, node.val, node.left.take());
            Self::traverse(map, node.val, node.right.take());
        }
    }
}
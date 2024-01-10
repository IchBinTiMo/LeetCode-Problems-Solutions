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
use std::collections::{HashMap, HashSet, VecDeque};
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        Solution::bfs(Solution::convert(root), start)
    }
    
    pub fn convert(root: Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, Vec<i32>> {
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut stack = VecDeque::new();
        stack.push_back(root.unwrap());

        while let Some(node) = stack.pop_front() {
            let nd = node.borrow();

            if let Some(left) = &nd.left {
                let l = left.borrow();

                graph.entry(nd.val).or_insert(Vec::new()).push(l.val);
                graph.entry(l.val).or_insert(Vec::new()).push(nd.val);
                stack.push_back(nd.left.clone().unwrap());
            }

            if let Some(right) = &nd.right {
                let r = right.borrow();

                graph.entry(nd.val).or_insert(Vec::new()).push(r.val);
                graph.entry(r.val).or_insert(Vec::new()).push(nd.val);
                stack.push_back(nd.right.clone().unwrap());
            }
        } 

        graph
    }

    pub fn bfs(graph: HashMap<i32, Vec<i32>>, start: i32) -> i32 {
        let mut stack: VecDeque<(i32, i32)> = VecDeque::new();
        let mut visited: HashSet<i32> = HashSet::new();
        let mut ans = 0;

        visited.insert(start);
        stack.push_back((start, 0));

        while let Some((current, depth)) = stack.pop_front() {
            if let Some(nodes) = graph.get(&current) {
                ans = ans.max(depth);
                for &node in nodes.iter() {
                    if visited.insert(node) {
                        stack.push_back((node, depth + 1));
                    }
                }
            }
        }

        ans
    }
}
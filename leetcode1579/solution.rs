impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        /// Union Find
        /// 
        /// Time: O(E + V) | Space: O(E + V)
        /// where E is the number of edges
        /// and V is the number of nodes
        let n: usize = n as usize;

        let mut size_a: Vec<i32> = vec![1; n];
        let mut size_b: Vec<i32> = vec![1; n];

        let mut parent_a: Vec<i32> = vec![-1; n];
        let mut parent_b: Vec<i32> = vec![-1; n];

        let mut res: i32 = 0;

        for edge in edges.iter() {
            // traverse edges with type 3 first because they are the only ones that Alice and Bob can share
            let tp: i32 = edge[0];
            let x: i32 = edge[1] - 1;
            let y: i32 = edge[2] - 1;

            let px_a: i32 = Self::find(&parent_a, x);
            let py_a: i32 = Self::find(&parent_a, y);
            let px_b: i32 = Self::find(&parent_b, x);
            let py_b: i32 = Self::find(&parent_b, y);

            if tp == 3 {
                if px_a == py_a && px_b == py_b {
                    res += 1;
                } else {
                    if px_a != py_a {
                        Self::unionn(&mut parent_a, &mut size_a, px_a, py_a);
                    }

                    if px_b != py_b {
                        Self::unionn(&mut parent_b, &mut size_b, px_b, py_b);
                    }
                }
            }
        }

        for edge in edges.iter() {
            // traverse edges with type 1 and 2
            let tp: i32 = edge[0];
            let x: i32 = edge[1] - 1;
            let y: i32 = edge[2] - 1;

            let px_a: i32 = Self::find(&parent_a, x);
            let py_a: i32 = Self::find(&parent_a, y);
            let px_b: i32 = Self::find(&parent_b, x);
            let py_b: i32 = Self::find(&parent_b, y);

            if tp == 2 {
                if px_b == py_b {
                    res += 1;
                } else {
                    Self::unionn(&mut parent_b, &mut size_b, px_b, py_b);
                }
            } else if tp == 1 {
                if px_a == py_a {
                    res += 1;
                } else {
                    Self::unionn(&mut parent_a, &mut size_a, px_a, py_a);
                }
            }
        }

        let mut self_parent = 0;

        // to check if there are more than 2 nodes with -1 parent
        // if so, some nodes are not connected
        for p in parent_a {
            if p == -1 {
                self_parent += 1;
            }
        }

        for p in parent_b {
            if p == -1 {
                self_parent += 1;
            }
        }

        return if self_parent > 2 {
            -1
        } else {
            res
        };

        
    }

    fn find(parent: &Vec<i32>, current: i32) -> i32 {
        return if parent[current as usize] == -1 {
                current
            } else {
                Self::find(parent, parent[current as usize])
            }
    }
    
    fn unionn(parent: &mut Vec<i32>, size: &mut Vec<i32>, x: i32, y: i32) {
        let x: usize = x as usize;
        let y: usize = y as usize;
        if size[x] < size[y] {
            parent[x] = y as i32;
            size[y] += size[x];
        } else {
            parent[y] = x as i32;
            size[x] += size[y];
        }
    }
}
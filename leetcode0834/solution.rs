use std::collections::HashMap;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = n as usize;

        let mut res: Vec<i32> = vec![0; n];
        let mut count: Vec<i32> = vec![0; n];

        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();

        for edge in edges.iter() {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;

            graph.entry(u).or_insert(vec![]).push(v);
            graph.entry(v).or_insert(vec![]).push(u);
        }

        Self::count_nodes(0,  usize::MAX, &graph, &mut count);

        let mut prev: Vec<usize> = Vec::new();

        let mut visited: Vec<bool> = vec![false; n];

        let mut step: i32 = 1;

        prev.push(0);

        loop {
            let mut current: Vec<usize> = Vec::new();

            while let Some(node) = prev.pop() {
                visited[node] = true;

                if let Some(nexts) = graph.get(&node) {
                    for &next in nexts.iter() {
                        if !visited[next] {
                            current.push(next);
                            res[0] += step;
                        }
                    }
                }
            }

            step += 1;
            prev = current;

            if prev.is_empty() {
                break;
            }
        }

        prev = Vec::new();
        visited = vec![false; n];

        prev.push(0);

        loop {
            let mut current: Vec<usize> = Vec::new();

            while let Some(node) = prev.pop() {
                visited[node] = true;

                if let Some(nexts) = graph.get(&node) {
                    for &next in nexts.iter() {
                        if !visited[next] {
                            res[next] = res[node] + (n as i32) - count[next] * 2;
                            current.push(next);
                        }
                    }
                }
            }

            prev = current;
            
            if prev.is_empty(){
                break;
            }
        }

        res
    }

    fn count_nodes(current: usize, prev: usize, graph: &HashMap<usize, Vec<usize>>, count: &mut Vec<i32>) {
        count[current] = 1;
        if let Some(nexts) = graph.get(&current) {
            for &next in nexts.iter() {
                if next != prev {
                    Self::count_nodes(next, current, graph, count);
                    count[current] += count[next];
                }
            }
        }
    }

}
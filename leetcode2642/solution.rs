const INF: i32 = i32::MAX / 3;

struct Graph {
    node_count: usize,
    floyd: Vec<Vec<i32>>,
}

impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut floyd = vec![vec![INF; n]; n];
        for i in 0..n {
            floyd[i][i] = 0;
        }
        for edge in edges.iter() {
            let (x, y, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
            floyd[x][y] = cost;
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    floyd[i][j] = floyd[i][j].min(floyd[i][k] + floyd[k][j]);
                }
            }
        }
        Self { node_count: n, floyd }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        let (x, y, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);
        for i in 0..self.node_count {
            for j in 0..self.node_count {
                self.floyd[i][j] = self.floyd[i][j].min(cost + self.floyd[i][x] + self.floyd[y][j]);
            }
        }
    }

    fn shortest_path(&self, x: i32, y: i32) -> i32 {
        let (x, y) = (x as usize, y as usize);
        if self.floyd[x][y] == INF {
            -1
        } else {
            self.floyd[x][y]
        }
    }
}
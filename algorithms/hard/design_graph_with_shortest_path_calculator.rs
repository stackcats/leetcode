struct Graph {
    dis: Vec<Vec<i32>>,
}


const max: i32 = 1000000000;

impl Graph {

    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        let n = n as usize;
        let mut dis = vec![vec![max; n]; n];

        for i in 0..n {
            dis[i][i] = 0;
        }

        for edges in edges {
            dis[edges[0] as usize][edges[1] as usize] = edges[2];
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    dis[i][j] = dis[i][j].min(dis[i][k] + dis[k][j]);
                }
            }
        }
        
        Self {
            dis: dis,
        }
    }
    
    fn add_edge(&mut self, edge: Vec<i32>) {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        let cost = edge[2];

        if self.dis[a][b] < cost {
            return;
        }

        self.dis[a][b] = cost;

        for i in 0..self.dis.len() {
            for j in 0..self.dis.len() {
                self.dis[i][j] = self.dis[i][j].min(self.dis[i][a] + cost + self.dis[b][j]);
            }
        }
    }
    
    fn shortest_path(&self, node1: i32, node2: i32) -> i32 {
        let n = self.dis[node1 as usize][node2 as usize];
        if n == max {
            -1
        } else {
            n
        }
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

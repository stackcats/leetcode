impl Solution {
    pub fn find_min_height_trees(mut n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        if n == 1 {
            return vec![0];
        }

        let mut adjacency_list = vec![vec![]; n as usize];
        let mut degrees = vec![0; n as usize];
        for edge in &edges {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            adjacency_list[n1].push(n2);
            adjacency_list[n2].push(n1);
            degrees[n1] += 1;
            degrees[n2] += 1;
        }

        let mut leaves: Vec<_> = degrees
            .iter()
            .enumerate()
            .filter(|&(_, d)| *d == 1)
            .map(|(i, _)| i)
            .collect();

        while n > 2 {
            n -= leaves.len() as i32;
            let mut new_leaves = Vec::new();
            for &leaf in &leaves {
                for &neighbor in &adjacency_list[leaf] {
                    degrees[neighbor] -= 1;
                    if degrees[neighbor] == 1 {
                        new_leaves.push(neighbor);
                    }
                }
            }
            leaves = new_leaves;
        }
        leaves.into_iter().map(|n| n as i32).collect()
    }
}

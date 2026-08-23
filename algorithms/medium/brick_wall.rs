use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut mp = HashMap::new();
        let n = wall.len() as i32;
        let mut max_edges = 0;
        for bricks in wall {
            let mut sum = 0;
            for i in 0..bricks.len() - 1 {
                sum += bricks[i] as i64;
                let ct = mp.entry(sum).or_insert(0);
                *ct += 1;
                max_edges = max_edges.max(*ct);
            }
        }
        n - max_edges
    }
}

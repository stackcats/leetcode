use std::collections::LinkedList;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let target = (graph.len() - 1) as i32;
        let mut q = LinkedList::new();
        let mut ans = Vec::new();
        q.push_back(vec![0]);
        while q.len() > 0 {
            let arr = q.front_back().unwrap();
            let source = arr[arr.len() - 1];
            for n in &graph[source as usize] {
                let mut v = arr.to_vec();
                v.push(*n);
                if *n == target {
                    ans.push(v);
                } else {
                    q.push_back(v);
                }
            }
        }
        ans
    }
}

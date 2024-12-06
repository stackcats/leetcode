use std::collections::{BTreeSet, VecDeque};

impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut mp = vec![vec![]; n];
        let mut degree = vec![0; n];
        for edge in edges {
            mp[edge[0] as usize].push(edge[1] as usize);
            degree[edge[1] as usize] += 1;
        }

        let mut q = VecDeque::new();
        for i in 0..n {
            if degree[i] == 0 {
                q.push_back(i);
            }
        }

        let mut arr = vec![BTreeSet::new(); n];

        while let Some(i) = q.pop_front() {
            for &m in &mp[i] {
                degree[m] -= 1;
                if degree[m] == 0 {
                    q.push_back(m);
                }
                arr[m] = arr[m].union(&arr[i]).copied().collect();
                arr[m].insert(i as i32);
            }
        }

        arr.into_iter().map(|x| x.into_iter().collect()).collect()
    }
}


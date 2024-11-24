use std::collections::{HashMap, VecDeque};

fn sort(k: i32, conditions: Vec<Vec<i32>>) -> Vec<i32> {
    let mut indegrees = HashMap::new();
    let mut neighours = HashMap::new();
    for c in conditions {
        *indegrees.entry(c[1]).or_insert(0) += 1;
        neighours.entry(c[0]).or_insert(vec![]).push(c[1]);
    }
    let mut q = VecDeque::new();
    let mut arr = Vec::new();
    for n in 1..=k {
        if *indegrees.entry(n).or_insert(0) == 0 {
            q.push_back(n);
        }
    }
    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        arr.push(n);
        if let Some(ns) = neighours.get(&n) {
            for &m in ns {
                let e = indegrees.entry(m).or_insert(0);
                *e -= 1;
                if *e == 0 {
                    q.push_back(m);
                }
            }
        }
    }
    arr
}

impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let row = sort(k, row_conditions);
        if row.len() != k as usize {
            return vec![];
        }

        let col = sort(k, col_conditions);
        if col.len() != k as usize {
            return vec![];
        }

        let mut mat = vec![vec![0; k as usize]; k as usize];
        let mut mp = HashMap::new();
        for (i, n) in col.into_iter().enumerate() {
            mp.insert(n, i);
        }

        for (i, n) in row.into_iter().enumerate() {
            mat[i][mp[&n]] = n;
        }

        mat
    }
}


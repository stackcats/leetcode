use std::collections::HashMap;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut in_degrees = vec![0; num_courses as usize];

        let mut edges = HashMap::new();
        for p in &prerequisites {
            let n1 = p[0];
            let n2 = p[1];
            in_degrees[n2 as usize] += 1;
            let v = edges.entry(n1).or_insert(vec![]);
            (*v).push(n2);
        }

        let mut q = Vec::new();
        for (i, d) in in_degrees.iter().enumerate() {
            if *d == 0 {
                q.push(i as i32);
            }
        }

        while !q.is_empty() {
            let n = q.pop().unwrap();
            if let Some((_, v)) = edges.remove_entry(&n) {
                for m in v {
                    in_degrees[m as usize] -= 1;
                    if in_degrees[m as usize] == 0 {
                        q.push(m);
                    }
                }
            }
        }

        edges.len() == 0
    }
}

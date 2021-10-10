use std::collections::HashMap;

fn aux(s: &str, map: &mut HashMap<String, Vec<String>>) {
    let mut iter = s.split_whitespace();
    let dirname = iter.next().unwrap();
    for file in iter {
        let mut name_content_iter = file.split(['(', ')'].as_ref());
        let filepath = format!("{}/{}", dirname, name_content_iter.next().unwrap());
        let content = name_content_iter.next().unwrap().to_string();
        (*map.entry(content).or_insert(vec![])).push(filepath);
    }
}

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        for p in &paths {
            aux(p, &mut map);
        }

        map.values()
            .filter_map(|v| if v.len() < 2 { None } else { Some(v.to_vec()) })
            .collect()
    }
}

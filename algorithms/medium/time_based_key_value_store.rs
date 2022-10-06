use std::collections::HashMap;

struct TimeMap {
    map: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let arr = self.map.entry(key).or_insert(vec![]);
        arr.push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(arr) = self.map.get(&key) {
            match arr.binary_search_by_key(&timestamp, |(a, b)| *a) {
                Ok(i) => arr[i].1.to_string(),
                Err(i) => {
                    if i == 0 {
                        "".to_string()
                    } else {
                        arr[i - 1].1.to_string()
                    }
                }
            }
        } else {
            "".to_string()
        }
    }
}

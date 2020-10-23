use std::collections::HashMap;
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut map = HashMap::new();
        for c in text.chars() {
            let counter = map.entry(c.clone()).or_insert(0);
            *counter += 1;
        }
        map.entry('b').or_insert(0);
        map.entry('a').or_insert(0);
        let counter = map.entry('l').or_insert(0);
        *counter /= 2;
        let counter = map.entry('o').or_insert(0);
        *counter /= 2;
        map.entry('n').or_insert(0);
        let mut arr = vec![map[&'b'], map[&'a'], map[&'l'], map[&'o'], map[&'n']];
        arr.sort();
        arr[0]
    }
}

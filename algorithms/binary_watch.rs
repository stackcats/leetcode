use std::collections::HashMap;

impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut h = HashMap::new();
        for n in 1_i32..=11 {
            h.entry(n.count_ones())
                .or_insert(vec![])
                .push(format!("{}", n));
        }
        let mut m = HashMap::new();
        for n in 1_i32..=59 {
            m.entry(n.count_ones())
                .or_insert(vec![])
                .push(format!("{}", n));
        }
        let mut ans = Vec::new();
        let hs = [0, 1, 2, 3];
        let ms = [0, 1, 2, 3, 4, 5];
        for x in hs.iter() {
            for y in ms.iter() {
                if x + y == num as u32 {
                    let hh = if *x == 0 {
                        vec!["0".to_string()]
                    } else {
                        h[x].clone()
                    };
                    let mm = if *y == 0 {
                        vec!["00".to_string()]
                    } else {
                        m[y].clone()
                    };
                    for a in hh.iter() {
                        for b in mm.iter() {
                            ans.push(format!("{}:{:0>2}", a, b));
                        }
                    }
                }
            }
        }
        ans
    }
}

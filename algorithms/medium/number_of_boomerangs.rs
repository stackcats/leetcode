use std::collections::HashMap;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..points.len() {
            let mut mp = HashMap::new();
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let [x1, y1] = points[i].as_slice() else {
                    unreachable!()
                };
                let [x2, y2] = points[j].as_slice() else {
                    unreachable!()
                };
                let (dx, dy) = (x1 - x2, y1 - y2);
                let distance = dx * dx + dy * dy;
                let ct = mp.entry(distance).or_insert(0);
                ans += *ct * 2;
                *ct += 1;
            }
        }
        ans
    }
}

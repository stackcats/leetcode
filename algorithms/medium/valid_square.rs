use std::collections::HashSet;

fn dis(p1: &[i32], p2: &[i32]) -> i32 {
    let x1 = p1[0] as f64;
    let y1 = p1[1] as f64;
    let x2 = p2[0] as f64;
    let y2 = p2[1] as f64;

    let dis = ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt();
    (dis * 100.0).round() as i32
}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let mut point_set = HashSet::new();
        point_set.insert(p1.clone());
        point_set.insert(p2.clone());
        point_set.insert(p3.clone());
        point_set.insert(p4.clone());

        if point_set.len() != 4 {
            return false;
        }

        let mut dis_set = HashSet::new();
        dis_set.insert(dis(&p1, &p2));
        dis_set.insert(dis(&p1, &p3));
        dis_set.insert(dis(&p1, &p4));
        dis_set.insert(dis(&p2, &p3));
        dis_set.insert(dis(&p2, &p4));
        dis_set.insert(dis(&p3, &p4));

        dis_set.len() == 2
    }
}

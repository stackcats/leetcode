impl Solution {
    pub fn max_ratings(units: Vec<Vec<i32>>) -> i64 {
        let mut m1 = i32::MAX;
        let mut m2 = i32::MAX;
        let mut s1 = 0;
        let mut s2 = 0;
        for unit in units {
            let mut t1 = i32::MAX;
            let mut t2 = i32::MAX;
            if unit.len() == 1 {
                m1 = m1.min(unit[0]);
                s1 += unit[0] as i64;
                continue;
            }
            for n in unit {
                if t1 >= n {
                    t2 = t1;
                    t1 = n;
                } else if t2 >= n {
                    t2 = n;
                }
            }
            s1 += t1 as i64;
            s2 += t2 as i64;
            m1 = m1.min(t1);
            m2 = m2.min(t2);
        }

        s1.max(s2 + (m1 - m2) as i64)
    }
}

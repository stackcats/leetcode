impl Solution {
    pub fn aggregate_time_series(series1: Vec<Vec<i32>>, series2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut ans = Vec::new();
        while i < series1.len() && j < series2.len() {
            let (t1, v1) = (series1[i][0], series1[i][1]);
            let (t2, v2) = (series2[j][0], series2[j][1]);
            ans.push(vec![t1.min(t2), v1 + v2]);
            if t1 == t2 {
                i += 1;
                j += 1;
            } else if t1 < t2 {
                i += 1;
            } else {
                j += 1;
            }
        }

        if i < series1.len() {
            ans.extend(series1.into_iter().skip(i));
        }

        if j < series2.len() {
            ans.extend(series2.into_iter().skip(j));
        }

        ans
    }
}

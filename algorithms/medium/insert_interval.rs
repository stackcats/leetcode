impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![new_interval];
        }

        let mut ans = Vec::new();

        let mut i = 0;
        while i < intervals.len() {
            if intervals[i][0] <= new_interval[0] && intervals[i][1] >= new_interval[1] {
                break;
            }

            if intervals[i][1] < new_interval[0] {
                ans.push(intervals[i].clone());
                i += 1;
                continue;
            }

            if intervals[i][0] > new_interval[1] {
                ans.push(new_interval.clone());
                break;
            }

            new_interval[0] = new_interval[0].min(intervals[i][0]);
            new_interval[1] = new_interval[1].max(intervals[i][1]);
            i += 1;
        }

        if i == intervals.len() {
            ans.push(new_interval.clone());
        }

        for j in i..intervals.len() {
            ans.push(intervals[j].clone());
        }

        ans
    }
}

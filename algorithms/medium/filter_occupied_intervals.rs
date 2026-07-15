fn is_overlap(a: &Vec<i32>, b: &Vec<i32>) -> bool {
    b[0] <= a[1] + 1
}

impl Solution {
    pub fn filter_occupied_intervals(
        mut occupied_intervals: Vec<Vec<i32>>,
        free_start: i32,
        free_end: i32,
    ) -> Vec<Vec<i32>> {
        occupied_intervals.sort_by_key(|v| v[0]);
        let mut arr = Vec::new();
        for v in occupied_intervals {
            if arr.is_empty() {
                arr.push(v);
            } else if is_overlap(arr.last().unwrap(), &v) {
                let last = arr.last_mut().unwrap();
                last[1] = last[1].max(v[1]);
            } else {
                arr.push(v);
            }
        }

        let mut ans = Vec::new();
        for v in arr {
            if v[0] < free_start {
                if v[1] < free_start {
                    ans.push(v);
                } else if v[1] <= free_end {
                    ans.push(vec![v[0], free_start - 1]);
                } else {
                    ans.push(vec![v[0], free_start - 1]);
                    ans.push(vec![free_end + 1, v[1]]);
                }
            } else if v[0] <= free_end {
                if v[1] <= free_end {
                    continue;
                } else {
                    ans.push(vec![free_end + 1, v[1]]);
                }
            } else {
                ans.push(v);
            }
        }
        ans
    }
}

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut j = 0;
        let mut ans = Vec::new();
        while i < first_list.len() && j < second_list.len() {
            let start_x = first_list[i][0];
            let end_x = first_list[i][1];
            let start_y = second_list[j][0];
            let end_y = second_list[j][1];
            if start_y > end_x {
                i += 1;
                continue;
            }
            if start_x > end_y {
                j += 1;
                continue;
            }
            ans.push(vec![start_x.max(start_y), end_x.min(end_y)]);
            if end_x > end_y {
                j += 1;
            } else {
                i += 1;
            }
        }
        ans
    }
}

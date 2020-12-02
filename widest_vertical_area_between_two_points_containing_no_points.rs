impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut arr = Vec::new();
        for each in &points {
            arr.push(each[0]);
        }
        arr.sort();
        let mut ans = 0;
        for i in 1..arr.len() {
            if arr[i] - arr[i - 1] > ans {
                ans = arr[i] - arr[i - 1];
            }
        }
        ans
    }
}

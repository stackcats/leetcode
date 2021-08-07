impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        arr.sort();
        let mut diff = (arr[0] - arr[1]).abs();
        ans.push(vec![arr[0], arr[1]]);
        for i in 1..(arr.len() - 1) {
            let mut t = (arr[i] - arr[i + 1]).abs();
            if diff > t {
                diff = t;
                ans.clear();
                ans.push(vec![arr[i], arr[i + 1]]);
            } else if diff == t {
                ans.push(vec![arr[i], arr[i + 1]]);
            }
        }
        ans
    }
}

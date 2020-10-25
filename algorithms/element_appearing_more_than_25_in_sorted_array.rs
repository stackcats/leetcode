impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let n = ((arr.len() as f64) * 0.25).floor();
        let mut i = 0;
        while i < arr.len() {
            let mut j = i + 1;
            while j < arr.len() && arr[j] == arr[i] {
                j += 1;
            }
            let l = j - i;
            if l as f64 > n {
                return arr[i];
            }
            i = j;
        }
        -1
    }
}

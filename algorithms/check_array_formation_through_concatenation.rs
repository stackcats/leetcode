use std::collections::HashSet;

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut checked = 0_usize;
        for i in 0..pieces.len() {
            let mut k = 0;
            while k < arr.len() && arr[k] != pieces[i][0] {
                k += 1;
            }
            let mut j = 0;
            while j < pieces[i].len() && k < arr.len() {
                if pieces[i][j] != arr[k] {
                    return false;
                }
                checked += 1;
                j += 1;
                k += 1
            }
        }
        checked == arr.len()
    }
}

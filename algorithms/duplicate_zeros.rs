impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut j = 0_i32;
        for i in 0..arr.len() {
            if arr[i] == 0 {
                j += 1;
            }
            j += 1;
        }
        let mut i = arr.len() - 1;
        j -= 1;
        while j >= 0 {
            if j < arr.len() as i32 {
                arr[j as usize] = arr[i];
            }
            if arr[i] == 0 {
                j -= 1;
                if j < arr.len() as i32 {
                    arr[j as usize] = 0;
                }
            }
            i -= 1;
            j -= 1;
        }
    }
}

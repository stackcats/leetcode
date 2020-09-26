impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let mut i = 0;
        let mut j = a.len() - 1;
        while i < j {
            while i < j && a[i] % 2 == 0 {
                i += 1;
            }
            while j > i && a[j] % 2 == 1 {
                j -= 1;
            }
            if i == j {
                break;
            }
            if i < j {
                a.swap(i, j);
            }
            i += 1;
            j -= 1;
        }
        a
    }
}

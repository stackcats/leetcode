// https://leetcode.com/problems/sort-array-by-parity-ii/description/

impl Solution {
    pub fn sort_array_by_parity_ii(a: &mut Vec<i32>) -> Vec<i32> {
        let mut b = vec![0; a.len()];
        let mut i = 0;
        let mut j = 1;
        while i < a.len() && j < a.len() {
            if a[i] % 2 == 0 {
                b[i] = a[i];
                i += 2;
            } else if a[j] % 2 == 1 {
                b[j] = a[j];
                j += 2;
            } else {
                b[i] = a[j];
                b[j] = a[i];
                i += 2;
                j += 2;
            }
        }

        while i < a.len() {
            b[i] = a[i];
            i += 2;
        }

        while j < a.len() {
            b[j] = a[j];
            j += 2;
        }

        b
    }
}

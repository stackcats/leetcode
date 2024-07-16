impl Solution {
    pub fn is_fascinating(n: i32) -> bool {
        let mut arr = vec![0; 10];
        let mut brr = vec![n, n * 2, n * 3];
        let mut i = 0;
        while i < 3 {
            while brr[i] > 0 {
                arr[(brr[i] % 10) as usize] += 1;
                brr[i] /= 10;
            }
            i += 1;
        }

        for i in 1..=9 {
            if arr[i as usize] != 1 {
                return false;
            }
        }
        arr[0] == 0
    }
}

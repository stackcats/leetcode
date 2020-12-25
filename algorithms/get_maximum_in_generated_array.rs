impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }
        let mut arr = vec![0; (n + 1) as usize];
        arr[1] = 1;
        let mut ans = arr[1];
        for i in 2..=(n as usize) {
            if i % 2 == 0 {
                arr[i] = arr[i / 2];
            } else {
                let h = i / 2;
                arr[i] = arr[h] + arr[h + 1];
            }
            ans = ans.max(arr[i]);
        }
        ans
    }
}

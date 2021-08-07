impl Solution {
    pub fn fair_candy_swap(a: Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {
        let alice: i32 = a.iter().sum();
        let bob: i32 = b.iter().sum();
        let t = (alice - bob) / 2;
        b.sort();
        for i in 0..a.len() {
            let mut l = 0;
            let mut r = (b.len() as i32) - 1;
            while l <= r {
                let mid = (l + r) / 2;
                if b[mid as usize] == a[i] - t {
                    return vec![a[i], b[mid as usize]];
                } else if b[mid as usize] < a[i] - t {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }

        vec![]
    }
}

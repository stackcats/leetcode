fn dfs(arr: &mut Vec<usize>, n: usize) -> i32 {
    if n == 0 {
        return 1;
    }

    let mut ans = 0;
    for i in 0..n {
        if arr[i] % n == 0 || n % arr[i] == 0 {
            arr.swap(i, n - 1);
            ans += dfs(arr, n - 1);
            arr.swap(i, n - 1);
        }
    }
    ans
}

impl Solution {
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let mut arr: Vec<usize> = (1..=n).collect();
        dfs(&mut arr, n)
    }
}

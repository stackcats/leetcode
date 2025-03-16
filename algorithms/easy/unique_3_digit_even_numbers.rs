impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut ct = vec![0; 10];
        for d in digits {
            ct[d as usize] += 1;
        }

        let mut ans = 0;
        'a: for n in (100..=998).step_by(2) {
            let a = n / 100;
            let b = n / 10 % 10;
            let c = n % 10;

            let mut arr = vec![0; 10];
            arr[a] += 1;
            arr[b] += 1;
            arr[c] += 1;

            for i in 0..10 {
                if arr[i] > ct[i] {
                    continue 'a;
                }
            }

            ans += 1;
        }

        ans
    }
}

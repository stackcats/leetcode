impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let radix: u32 = 10;
        let ar: Vec<u32> = a
            .chars()
            .rev()
            .map(|c| c.to_digit(radix).unwrap())
            .collect();
        let br: Vec<u32> = b
            .chars()
            .rev()
            .map(|c| c.to_digit(radix).unwrap())
            .collect();
        let mut ans = Vec::new();
        let mut prime = 0;
        let mut i = 0;
        let mut j = 0;
        while i < ar.len() && j < br.len() {
            let n = ar[i] + br[j] + prime;
            ans.push(n % 2);
            prime = n / 2;
            i += 1;
            j += 1;
        }
        while i < ar.len() {
            let n = ar[i] + prime;
            ans.push(n % 2);
            prime = n / 2;
            i += 1;
        }
        while j < br.len() {
            let n = br[j] + prime;
            ans.push(n % 2);
            prime = n / 2;
            j += 1;
        }
        if prime > 0 {
            ans.push(prime);
        }
        ans.iter()
            .rev()
            .map(|n| std::char::from_digit(*n, radix).unwrap())
            .collect()
    }
}

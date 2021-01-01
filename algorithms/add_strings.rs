impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut p = 0;
        let z = '0' as u8;
        let n1: Vec<u8> = num1.chars().rev().map(|c| c as u8 - z).collect();
        let n2: Vec<u8> = num2.chars().rev().map(|c| c as u8 - z).collect();
        let mut i = 0;
        let mut j = 0;
        let mut ans = Vec::new();
        while i < n1.len() && j < n2.len() {
            let n = n1[i] + n2[j] + p;
            ans.push(n % 10);
            p = n / 10;
            i += 1;
            j += 1;
        }
        while i < n1.len() {
            let n = n1[i] + p;
            ans.push(n % 10);
            p = n / 10;
            i += 1;
        }
        while j < n2.len() {
            let n = n2[j] + p;
            ans.push(n % 10);
            p = n / 10;
            j += 1;
        }
        if p == 1 {
            ans.push(1);
        }
        ans.into_iter().rev().map(|u| (u + z) as char).collect()
    }
}

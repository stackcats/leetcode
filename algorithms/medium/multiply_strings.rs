impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let num1: Vec<u8> = num1.as_bytes().iter().map(|b| b - b'0').collect();
        let num2: Vec<u8> = num2.as_bytes().iter().map(|b| b - b'0').collect();
        let mut arr = vec![0; num1.len() + num2.len()];
        for (i, n1) in num1.iter().rev().enumerate() {
            for (j, n2) in num2.iter().rev().enumerate() {
                arr[i + j] += n1 * n2;
                arr[j + i + 1] += arr[i + j] / 10;
                arr[i + j] %= 10;
            }
        }

        arr.into_iter()
            .rev()
            .skip_while(|x| *x == 0)
            .map(|x| (b'0' + x) as char)
            .collect()
    }
}

impl Solution {
    pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
        let rows = rows as usize;
        let cols = encoded_text.len() / rows;
        let mut mat = vec![vec![' '; cols]; rows];
        let s = encoded_text.chars().collect::<Vec<_>>();
        let mut k = 0;
        for i in 0..rows {
            for j in 0..cols {
                mat[i][j] = s[k];
                k += 1;
            }
        }

        let mut ans = String::new();
        for j in 0..cols {
            let mut a = 0;
            let mut b = j;
            while a < rows && b < cols {
                ans.push(mat[a][b]);
                a += 1;
                b += 1;
            }
        }
        ans.trim_right().to_string()
    }
}

impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort();
        let mut ans = Vec::new();
        let mut prefix = String::new();
        let mut start = 0;
        for c in search_word.chars() {
            prefix.push(c);
            start = start + products[start..].binary_search(&prefix).unwrap_or_else(|p| p);
            let mut arr = Vec::new();
            let end = products.len().min(start + 3);
            for i in start..end {
                if products[i].starts_with(&prefix) {
                    arr.push(products[i].to_string());
                }
            }
            
            ans.push(arr);
        }
        ans
    }
}

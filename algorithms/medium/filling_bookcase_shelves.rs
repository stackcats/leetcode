impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let mut dp = vec![0; books.len() + 1];
        for i in 0..books.len() {
            dp[i + 1] = dp[i] + books[i][1];
            let mut total_width = 0;
            let mut max_height = 0;
            for j in (0..=i).rev() {
                total_width += books[j][0];
                if total_width > shelf_width {
                    break;
                }
                max_height = max_height.max(books[j][1]);
                dp[i + 1] = dp[i + 1].min(dp[j] + max_height);
            }
        }
        dp[books.len()]
    }
}

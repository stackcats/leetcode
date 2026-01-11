fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
    let mut st = Vec::new();
    let mut ans = 0;
    for (i, h) in heights.iter().enumerate() {
        while let Some(t) = st.last()
            && heights[*t] > *h
        {
            let j = st.pop().unwrap();
            let w = if st.is_empty() {
                i
            } else {
                i - st[st.len() - 1] - 1
            };
            ans = ans.max(heights[j] * w as i32);
        }
        st.push(i);
    }

    ans
}

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut heights = vec![0; matrix[0].len() + 1];
        let mut ans = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == '0' {
                    heights[j] = 0;
                } else {
                    heights[j] += 1;
                }
            }
            let area = largest_rectangle_area(&heights);
            ans = ans.max(area);
        }

        ans
    }
}

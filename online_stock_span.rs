// https://leetcode.com/problems/online-stock-span/

struct StockSpanner {
    st: Vec<(i32, i32)>,
}

/** You can modify the type of `self` for your need. */
impl StockSpanner {
    fn new() -> Self {
        StockSpanner { st: vec![] }
    }

    fn next(&mut self, price: i32) -> i32 {
        if self.st.len() == 0 {
            self.st.push((1, price));
            return 1;
        }

        let mut curr = (1, price);
        while self.st.len() > 0 && self.st[self.st.len() - 1].1 <= price {
            let top = self.st.pop().unwrap();
            curr.0 += top.0;
        }

        self.st.push(curr);

        curr.0
    }
}

struct Expr {
    tokens: Vec<u8>,
    i: usize,
}

impl Expr {
    fn new(tokens: Vec<u8>) -> Self {
        Self { tokens, i: 0 }
    }
    fn parse(&mut self) -> bool {
        let r = match self.tokens[self.i] {
            b't' => true,
            b'f' => false,
            b'!' => {
                self.i += 2;
                !self.parse()
            }
            b'&' => {
                self.i += 2;
                self.parse_list().into_iter().all(|x| x)
            }
            b'|' => {
                self.i += 2;
                self.parse_list().into_iter().any(|x| x)
            }
            _ => {
                unreachable!()
            }
        };

        self.i += 1;
        r
    }

    fn parse_list(&mut self) -> Vec<bool> {
        let mut v = Vec::new();
        loop {
            let sub = self.parse();
            v.push(sub);
            if self.i >= self.tokens.len() || self.tokens[self.i] != b',' {
                break;
            }
            self.i += 1;
        }
        v
    }
}

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut expr = Expr::new(expression.into_bytes());
        expr.parse()
    }
}

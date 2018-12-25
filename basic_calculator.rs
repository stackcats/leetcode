// https://leetcode.com/problems/basic-calculator/

struct Token {
    tokens: Vec<String>,
    ndx: usize,
}

impl Token {
    fn new(s: &str) -> Self {
        let mut tks = Vec::new();
        let mut iter = s.chars().peekable();

        while let Some(ch) = iter.next() {
            if ch == ' ' {
            } else if ch == '+' {
                tks.push("+".to_string());
            } else if ch == '-' {
                tks.push("-".to_string());
            } else if ch == '(' {
                tks.push("(".to_string());
            } else if ch == ')' {
                tks.push(")".to_string());
            } else if ch.is_numeric() {
                let mut nums: Vec<char> = Vec::new();
                nums.push(ch);

                loop {
                    if let Some(d) = iter.peek() {
                        if d.is_numeric() {
                            nums.push(*d);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }

                    iter.next();
                }

                let n: String = nums.iter().cloned().collect();
                tks.push(n);
            }
        }

        Token {
            ndx: 0,
            tokens: tks,
        }
    }

    fn get(&mut self) -> Option<String> {
        if self.ndx >= self.tokens.len() {
            return None;
        }

        let s = self.tokens[self.ndx].clone();
        self.ndx += 1;

        Some(s)
    }

    fn unget(&mut self) {
        self.ndx -= 1;
    }
}

fn parse_expression(tks: &mut Token) -> i32 {
    let mut v1 = parse_term(tks);
    loop {
        let otk = tks.get();
        if otk.is_none() {
            break;
        }

        let tk = otk.unwrap();
        if tk != "+" && tk != "-" {
            tks.unget();
            break;
        }

        let v2 = parse_term(tks);
        if tk == "+" {
            v1 += v2;
        } else if tk == "-" {
            v1 -= v2;
        } else {
            tks.unget();
        }
    }

    v1
}

fn parse_term(tks: &mut Token) -> i32 {
    let mut tk = tks.get().unwrap();
    let mut sign = 1;
    if tk == "-" {
        sign = -1;
    } else {
        tks.unget();
    }

    tk = tks.get().unwrap();

    let mut v = 0;
    if let Ok(n) = tk.parse::<i32>() {
        v = n;
    } else if tk == "(" {
        v = parse_expression(tks);
        tks.get(); // ")"
    } else {
        tks.unget();
    }

    v * sign
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut tks = Token::new(&s);
        parse_expression(&mut tks)
    }
}

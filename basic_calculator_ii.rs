// https://leetcode.com/problems/basic-calculator-ii/

// 中缀表达式转后缀表达式

fn lex(s: &str) -> Vec<String> {
    let mut tks = Vec::new();
    let mut iter = s.chars().peekable();

    while let Some(ch) = iter.next() {
        if ch == ' ' {
        } else if ch == '+' {
            tks.push("+".to_string());
        } else if ch == '-' {
            tks.push("-".to_string());
        } else if ch == '*' {
            tks.push("*".to_string());
        } else if ch == '/' {
            tks.push("/".to_string());
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

    tks
}

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let tks = lex(&s);
        let mut post = Vec::new();
        let mut st = Vec::new();

        for s in tks.iter() {
            if s == "+" || s == "-" {
                while st.len() > 0 {
                    post.push(st.pop().unwrap());
                }
                st.push(s);
            } else if s == "*" || s == "/" {
                while st.len() > 0 {
                    if st[st.len() - 1] == "*" || st[st.len() - 1] == "/" {
                        post.push(st.pop().unwrap());
                    } else {
                        break;
                    }
                }
                st.push(s);
            } else {
                post.push(s);
            }
        }

        while st.len() > 0 {
            post.push(st.pop().unwrap());
        }

        let mut st = Vec::new();

        for &s in post.iter() {
            if s == "*" {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
                st.push(a * b);
            } else if s == "/" {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
                st.push(b / a);
            } else if s == "+" {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
                st.push(b + a);
            } else if s == "-" {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
                st.push(b - a);
            } else {
                st.push(s.parse::<i32>().unwrap());
            }
        }

        st.pop().unwrap()
    }
}

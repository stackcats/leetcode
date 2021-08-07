// https://leetcode.com/problems/evaluate-reverse-polish-notation/submissions/

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut st = Vec::new();

        for token in tokens.iter() {
            match token.as_ref() {
                "+" => {
                    let b = st.pop().unwrap();
                    let a = st.pop().unwrap();
                    st.push(a + b);
                }
                "-" => {
                    let b = st.pop().unwrap();
                    let a = st.pop().unwrap();
                    st.push(a - b);
                }
                "*" => {
                    let b = st.pop().unwrap();
                    let a = st.pop().unwrap();
                    st.push(a * b);
                }
                "/" => {
                    let b = st.pop().unwrap();
                    let a = st.pop().unwrap();
                    st.push(a / b);
                }
                _ => {
                    st.push(token.parse().unwrap());
                }
            }
        }

        st.pop().unwrap()
    }
}

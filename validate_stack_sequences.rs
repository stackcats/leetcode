// https://leetcode.com/problems/validate-stack-sequences/

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut st = Vec::new();

        let mut i = 0;
        let mut j = 0;
        while i < pushed.len() && j < popped.len() {
            if st.len() > 0 && st[st.len() - 1] == popped[j] {
                st.pop();
                j += 1;
            } else {
                if pushed[i] == popped[j] {
                    j += 1;
                } else {
                    st.push(pushed[i]);
                }
                i += 1;
            }
        }

        while st.len() > 0 && j < popped.len() {
            if st.len() > 0 && st[st.len() - 1] == popped[j] {
                st.pop();
                j += 1;
            } else {
                return false;
            }
        }

        st.len() == 0
    }
}

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn minimized_string_length(s: String) -> i32 {
        let st: HashSet<u8> = HashSet::from_iter(s.bytes().into_iter());
        st.len() as i32
    }
}

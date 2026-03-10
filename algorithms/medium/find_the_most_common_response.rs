use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_common_response(responses: Vec<Vec<String>>) -> String {
        let mut mp = HashMap::new();
        for resp in responses {
            let mut st = HashSet::new();
            for r in resp {
                st.insert(r);
            }

            for r in st {
                *mp.entry(r.to_string()).or_insert(0) += 1;
            }
        }

        let mut ans = String::new();
        let mut n = 0;
        for (k, v) in mp {
            if n < v {
                ans = k;
                n = v;
            } else if n == v && ans > k {
                ans = k;
            }
        }

        ans
    }
}

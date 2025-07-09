use std::cmp::Ordering;

fn is_valid(s: &str) -> bool {
    s != "" && s.chars().all(|c| c == '_' || c.is_ascii_alphanumeric())
}

fn to_order(s: &str) -> i32 {
    match s {
        "electronics" => 1,
        "grocery" => 2,
        "pharmacy" => 3,
        "restaurant" => 4,
        _ => 0,
    }
}

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut arr = Vec::new();
        for (i, c) in code.into_iter().enumerate() {
            if !is_valid(&c) || !is_active[i] {
                continue;
            }
            if to_order(&business_line[i]) == 0 {
                continue;
            }

            arr.push((c, business_line[i].to_string()));
        }

        arr.sort_by(|(a1, b1), (a2, b2)| match to_order(b1).cmp(&to_order(b2)) {
            Ordering::Equal => a1.cmp(a2),
            e => e,
        });
        arr.into_iter().map(|(a, b)| a).collect::<Vec<_>>()
    }
}

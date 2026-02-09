fn rev<F>(v: &mut [char], f: F)
where
    F: Fn(char) -> bool,
{
    let mut i = 0;
    let mut j = v.len() - 1;
    while i < j {
        while i < j && f(v[i]) {
            i += 1;
        }
        while j > i && j > 0 && f(v[j]) {
            j -= 1;
        }

        if i < j {
            v.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

impl Solution {
    pub fn reverse_by_type(s: String) -> String {
        let mut v = s.chars().collect::<Vec<_>>();
        rev(&mut v, |c: char| c.is_alphabetic());
        rev(&mut v, |c: char| !c.is_alphabetic());
        v.into_iter().collect::<String>()
    }
}

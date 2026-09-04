use regex::Regex;

fn simplify(s: &str) -> (i32, i32) {
    let re = Regex::new(r"(-?x|-?\d+x|-?\d+)").unwrap();
    re.find_iter(s).fold((0, 0), |(x, a), m| {
        let m = m.as_str();
        if m == "x" {
            (x + 1, a)
        } else if m == "-x" {
            (x - 1, a)
        } else if m.contains("x") {
            let mut n = m[..m.len() - 1].parse::<i32>().unwrap_or(1);
            (x + n, a)
        } else {
            let n = m.parse::<i32>().unwrap();
            (x, a + n)
        }
    })
}

impl Solution {
    pub fn solve_equation(equation: String) -> String {
        let v = equation.split("=").collect::<Vec<_>>();
        let (x1, a) = simplify(v[0]);
        let (x2, b) = simplify(v[1]);

        let mut x = x1 - x2;
        let mut c = b - a;

        if x == 0 && c == 0 {
            return "Infinite solutions".to_string();
        }

        if x == 0 {
            return "No solution".to_string();
        }

        format!("x={}", c / x)
    }
}

use std::convert::From;
use std::ops::Mul;
use std::string::ToString;

struct Complex(i32, i32);

impl From<String> for Complex {
    fn from(num: String) -> Self {
        let num = num.replace("i", "");
        let arr: Vec<_> = num.split("+").map(|s| s.parse::<i32>().unwrap()).collect();
        Self(arr[0], arr[1])
    }
}

impl ToString for Complex {
    fn to_string(&self) -> String {
        format!("{}+{}i", self.0, self.1)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let real = self.0 * rhs.0 - self.1 * rhs.1;
        let image = self.0 * rhs.1 + self.1 * rhs.0;
        Self(real, image)
    }
}

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let c1 = Complex::from(num1);
        let c2 = Complex::from(num2);
        (c1 * c2).to_string()
    }
}

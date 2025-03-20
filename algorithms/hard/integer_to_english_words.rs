fn get(n: i32) -> String {
    let s = match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        13 => "Thirteen",
        14 => "Fourteen",
        15 => "Fifteen",
        16 => "Sixteen",
        17 => "Seventeen",
        18 => "Eighteen",
        19 => "Nineteen",
        20 => "Twenty",
        30 => "Thirty",
        40 => "Forty",
        50 => "Fifty",
        60 => "Sixty",
        70 => "Seventy",
        80 => "Eighty",
        90 => "Ninety",
        _ => "",
    };
    s.to_string()
}

fn f(mut num: i32) -> String {
    let mut arr = Vec::new();
    let a = num / 100;
    num = num % 100;

    if a > 0 {
        arr.push(get(a));
        arr.push("Hundred".to_string());
    }

    if num == 0 {
    } else if num <= 20 {
        arr.push(get(num));
    } else {
        let b = num % 10;
        num -= b;
        arr.push(get(num));
        if b > 0 {
            arr.push(get(b));
        }
    }
    arr.join(" ")
}

impl Solution {
    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let mut arr = Vec::new();
        while num > 0 {
            arr.push(num % 1000);
            num /= 1000;
        }

        let mut div = ["", "Thousand", "Million", "Billion"];
        let mut ans = Vec::new();
        for i in 0..arr.len() {
            if arr[i] == 0 {
                continue;
            }

            if i > 0 {
                ans.push(div[i].to_string());
            }
            ans.push(f(arr[i]));
        }

        ans.reverse();

        ans.join(" ")
    }
}

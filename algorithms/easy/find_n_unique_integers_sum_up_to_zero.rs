impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        if n == 1 {
            return vec![0]
        }
        let mut arr = Vec::new();
        let end = n / 2;
        let mut start = -end;
        while start < 0 {
            arr.push(start);
            start += 1;
        }
        if  n % 2 != 0 {
            arr.push(0);
        }
        start = 1;
        while start <= end {
           arr.push(start);
            start += 1;            
        }
        arr
    }
}

fn score(arr: Vec<i32>) -> i32 {
    let mut p2 = 0;
    let mut p1 = 0;
    let mut s = 0;
    for n in arr {
        s += n;
        if p1 == 10 || p2 == 10 {
            s += n;
        }
        p2 = p1;
        p1 = n;
    }
    s
}

impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let s1 = score(player1);
        let s2 = score(player2);
        if s1 > s2 {
            1
        } else if s1 < s2 {
            2
        } else {
            0
        }
    }
}

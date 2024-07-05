impl Solution {
    pub fn dist_money(mut money: i32, mut children: i32) -> i32 {
        if money < children {
            return -1;
        }

        let mut ct = 0;
        while money >= 8 && children >= 1 && money > children {
            ct += 1;
            money -= 8;
            children -= 1;
        }

        if money < children || money > 0 && children == 0 || money == 4 && children == 1 {
            ct -= 1;
        }

        ct
    }
}

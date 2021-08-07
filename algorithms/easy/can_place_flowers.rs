impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut can_planted = 0;
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 1 {
                continue;
            }
            if i == 0 {
                if i + 1 < flowerbed.len() && flowerbed[i + 1] == 0 {
                    can_planted += 1;
                    flowerbed[i] = 1;
                } else if i + 1 == flowerbed.len() {
                    can_planted += 1;
                    flowerbed[i] = 1;
                }
            } else if i == flowerbed.len() - 1 {
                if flowerbed[i - 1] == 0 {
                    can_planted += 1;
                    flowerbed[i] = 1;
                }
            } else {
                if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
                    can_planted += 1;
                    flowerbed[i] = 1;
                }
            }
        }
        can_planted >= n
    }
}

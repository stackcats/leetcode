/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(mut n: i32) -> i32 {
        let mut s = 1;
        while s <= n {
            let m = s + (n - s) / 2;
            let r = guess(m);
            if r == 0 {
                return m;
            }
            if r == -1 {
                n = m - 1;
            } else {
                s = m + 1;
            }
        }
        0
    }
}

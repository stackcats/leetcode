// https://leetcode.com/problems/powx-n/description/

double myPow(double x, int n) {
  if (n == 0)
    return 1;

  if (n < 0) {
    x = 1 / x;
    n *= -1;
  }

  double res = x;
  n -= 1;

  while (n > 0) {
    if (n % 2 == 0) {
      x *= x;
      n /= 2;
    } else {
      res *= x;
      n -= 1;
    }
  }

  return res;
}

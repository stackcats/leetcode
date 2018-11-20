// https://leetcode.com/problems/sqrtx/description/

// 牛顿迭代
const double e = 0.1;

int mySqrt(int x) {
  double i = 1;
  while (fabs(x - i * i) > e) {
    i = (i + x / i) / 2;
  }
  return (int)i;
}

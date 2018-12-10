// https://leetcode.com/problems/divide-two-integers/description/

int divide(int dividend, int divisor) {
  if (dividend == 0) {
    return 0;
  }

  if (dividend == INT_MIN && divisor == -1) {
    return INT_MAX;
  }

  long long a = dividend;
  long long b = divisor;

  int sign = 1;
  if ((a < 0 && b > 0) || (a > 0 && b < 0)) {
    sign = -1;
  }

  a = llabs(a);
  b = llabs(b);

  long long ans = 0;

  while (a >= b) {
    long long ct = 1;
    long long c = b;
    while (a >= (c << 1)) {
      c = c << 1;
      ct = ct << 1;
    }
    ans += ct;
    a -= c;
  }
  return sign * ans;
}

// https://leetcode.com/problems/reverse-integer/description/

int reverse(int x) {
  int sign = x < 0 ? -1 : 1;
  int res = 0;
  x *= sign;

  while (x > 0) {
    unsigned long next = (unsigned long)res * 10 + x % 10;
    if (next > 0x7FFFFFFF)
      return 0;

    res = next;
    x /= 10;
  }
  return res * sign;
}

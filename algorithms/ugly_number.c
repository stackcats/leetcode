// https://leetcode.com/problems/ugly-number/description/

bool isUgly(int num) {
  if (num <= 0) {
    return 0;
  }

  if (num == 1) {
    return 1;
  }

  int factor = 2;
  while (num > 1) {
    if (num % factor == 0) {
      num /= factor;
    } else {
      factor++;
      if (factor > 5) {
        return 0;
      }
    }
  }

  return 1;
}

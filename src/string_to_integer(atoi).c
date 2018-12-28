// https://leetcode.com/problems/string-to-integer-atoi/description/

int myAtoi(char *s) {
  while (*s == ' ') {
    s++;
  }
  int sign = 1;
  if (*s == '-') {
    sign = -1;
    s++;
  } else if (*s == '+') {
    s++;
  }

  int ans = 0;
  while (*s != '\0') {
    if (isdigit(*s)) {
      if (ans > INT_MAX / 10) {
        return sign > 0 ? INT_MAX : INT_MIN;
      }
      ans *= 10;

      if (ans > INT_MAX - (*s - '0')) {
        return sign > 0 ? INT_MAX : INT_MIN;
      }

      ans += *s - '0';
      s++;
    } else {
      break;
    }
  }

  return ans * sign;
}

// https://leetcode.com/problems/valid-number/description/

bool isNumber(char *s) {
  // 去掉行首空格
  while (*s == ' ') {
    s++;
  }

  // 空字符串
  if (*s == '\0') {
    return 0;
  }

  // 去掉行尾空格
  int end = strlen(s) - 1;
  while (s[end] == ' ') {
    end--;
  }
  s[end + 1] = '\0';

  // 检查整数符号位
  if (*s == '-' || *s == '+') {
    s++;
  }

  int integerLen = 0; // 整数部分长度
  // 检查整数部分
  while (*s != '\0' && *s != '.' && *s != 'e') {
    if (!isdigit(*s)) {
      return 0;
    }
    integerLen++;
    s++;
  }

  // 只有整数部分
  if (*s == '\0') {
    return 1;
  }

  // 检查整数部分是否出现指数部分
  if (*s == 'e') {
    s++;

    // 检查指数符号位
    if (*s == '-' || *s == '+') {
      s++;
    }

    int expLen = 0; // 指数部分长度
    while (*s != '\0') {
      if (!isdigit(*s)) {
        return 0;
      }
      expLen++;
      s++;
    }

    // 不允许 e3 或者 3e的形式
    return integerLen > 0 && expLen > 0;
  }

  // 检查是否出现小数部分
  if (*s == '.') {
    s++;
  }

  // 检查是否以'.'结尾
  if (*s == '\0') {
    return integerLen > 0;
  }

  int decimalLen = 0; // 小数部分长度
  while (*s != '\0' && *s != 'e') {
    if (!isdigit(*s)) {
      return 0;
    }

    decimalLen++;
    s++;
  }

  // 判断小数位是否出现指数
  if (*s == 'e') {
    s++;
    if (*s == '-' || *s == '+') {
      s++;
    }

    int expLen = 0;
    while (*s != '\0') {
      if (!isdigit(*s)) {
        return 0;
      }
      expLen++;
      s++;
    }

    return (integerLen > 0 || decimalLen > 0) && expLen > 0;
  }

  return 1;
}

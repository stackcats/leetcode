// https://leetcode.com/problems/isomorphic-strings/description/

bool isIsomorphic(char *s, char *t) {
  // 字符数组当hash用
  char sh[256]; // 字符串s中的替换关系
  char th[256]; // 字符串t中的替换关系

  memset(sh, 0, sizeof(sh));
  memset(th, 0, sizeof(sh));

  while (*s != '\0' && *t != '\0') {
    if (sh[*s] == 0 && th[*t] == 0) {
      sh[*s] = *t;
      th[*t] = *s;
    } else if (sh[*s] != *t || th[*t] != *s) {
      return 0;
    }
    s++;
    t++;
  }

  return 1;
}

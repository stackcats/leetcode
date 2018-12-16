// https://leetcode.com/problems/find-the-difference/description/

// 找出唯一一个不同的元素 使用异或
char findTheDifference(char *s, char *t) {
  char r = 0;

  int i = 0;
  for (i = 0; s[i]; i++) {
    r ^= s[i] ^ t[i];
  }

  return r ^ t[i];
}

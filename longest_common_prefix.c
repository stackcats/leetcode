char *longestCommonPrefix(char **strs, int strsSize) {
  int len = strlen(strs[0]) + 1;
  char *res = (char *)malloc(sizeof(char) * len);
  memset(res, 0, sizeof(char) * len);
  for (int j = 0; j < len; j++) {
    int flag = 1;
    char c = strs[0][j];
    for (int i = 1; i < strsSize; i++) {
      if (strs[i][j] != c) {
        res[j] = '\0';
        return res;
      }
    }
    res[j] = c;
  }

  // O(n^2)
  return res;
}

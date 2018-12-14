// https://leetcode.com/problems/longest-palindromic-subsequence/description/

// d[i][j] 表示s[i]到s[j]中的最长回文子序列

// d[i][i] = 1
// d[i][j] = d[i+1][j-1] + 2 (s[i] == s[j])
// d[i][j] = max(d[i+1][j], d[i][j-1])

inline max(int a, int b) { return a < b ? b : a; }

int longestPalindromeSubseq(char *s) {
  int len = strlen(s);
  int d[len][len];
  for (int i = 0; i < len; i++) {
    memset(d + i, 0, sizeof(int) * len);
    d[i][i] = 1;
  }

  for (int i = len - 1; i >= 0; i--) {
    for (int j = i + 1; j < len; j++) {
      if (s[i] == s[j]) {
        d[i][j] = 2 + d[i + 1][j - 1];
      } else {
        d[i][j] = max(d[i + 1][j], d[i][j - 1]);
      }
    }
  }

  int maxLen = d[0][len - 1];

  return maxLen;
}

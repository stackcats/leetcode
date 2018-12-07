// https://leetcode.com/problems/implement-strstr/description/
void prefix_table(char *pattern, int *prefix, int size) {
  prefix[0] = 0;
  int i = 1;
  int j = prefix[0];
  while (i < size) {
    if (pattern[i] == pattern[j]) {
      j++;
      prefix[i] = j;
      i++;
    } else if (j == 0) {
      prefix[i] = 0;
      i++;
    } else {
      j = prefix[j - 1];
    }
  }
}

int kmp(char *s, int slen, char *p, int plen) {
  int i = 0;
  int j = 0;

  int *prefix = calloc(plen, sizeof(int));
  prefix_table(p, prefix, plen);

  while (i < slen && j < plen) {
    if (s[i] == p[j]) {
      j++;
    } else if (j != 0) {
      j = prefix[j - 1];
      continue;
    }
    i++;
  }

  free(prefix);
  return j == plen ? i - j : -1;
}

int strStr(char *haystack, char *needle) {
  int plen = strlen(needle);
  int slen = strlen(haystack);
  return kmp(haystack, slen, needle, plen);
}

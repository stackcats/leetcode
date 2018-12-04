// https://leetcode.com/problems/di-string-match/description/

/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *diStringMatch(char *S, int *returnSize) {
  int len = strlen(S);
  int *arr = calloc(len + 1, sizeof(int));
  *returnSize = len + 1;

  int D = len;
  int I = 0;
  int i;
  for (i = 0; i < len; i++) {
    if (S[i] == 'I') {
      arr[i] = I++;
    } else {
      arr[i] = D--;
    }
  }

  arr[i] = I;
  return arr;
}

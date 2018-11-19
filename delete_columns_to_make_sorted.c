// https://leetcode.com/problems/delete-columns-to-make-sorted/description/
int minDeletionSize(char **A, int ASize) {
  int res = 0;
  int column = strlen(A[0]);
  for (int j = 0; j < column; j++) {
    int pre = A[0][j];
    for (int i = 0; i < ASize; i++) {
      if (pre > A[i][j]) {
        res++;
        break;
      }
      pre = A[i][j];
    }
  }
  return res;
}

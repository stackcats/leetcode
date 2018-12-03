// https://leetcode.com/problems/pascals-triangle/description/

/**
 * Return an array of arrays.
 * The sizes of the arrays are returned as *columnSizes array.
 * Note: Both returned array and *columnSizes array must be malloced, assume
 * caller calls free().
 */
int **generate(int numRows, int **columnSizes) {
  int **arr = (int **)malloc(sizeof(int *) * numRows);
  *columnSizes = (int *)malloc(sizeof(int) * numRows);

  for (int i = 0; i < numRows; i++) {
    int len = i + 1;
    (*columnSizes)[i] = len;
    arr[i] = (int *)malloc(sizeof(int) * len);
    for (int j = 0; j < len; j++) {
      if (j == 0 || j == len - 1) {
        arr[i][j] = 1;
      } else {
        arr[i][j] = arr[i - 1][j - 1] + arr[i - 1][j];
      }
    }
  }
  return arr;
}

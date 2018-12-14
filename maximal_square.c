// https://leetcode.com/problems/maximal-square/description/

inline int max(int a, int b) { return a < b ? b : a; }

inline int min(int a, int b) { return a < b ? a : b; }

int maximalSquare(char **matrix, int matrixRowSize, int matrixColSize) {
  if (matrixRowSize == 0 || matrixColSize == 0) {
    return 0;
  }

  int maxLength = 0;

  int d[matrixRowSize][matrixColSize];

  for (int i = 0; i < matrixRowSize; i++) {
    for (int j = 0; j < matrixColSize; j++) {
      if (i == 0 || j == 0) {
        d[i][j] = matrix[i][j] - '0';
      } else if (matrix[i][j] == '1') {
        d[i][j] = min(d[i - 1][j - 1], min(d[i - 1][j], d[i][j - 1])) + 1;
      } else {
        d[i][j] = 0;
      }

      maxLength = max(maxLength, d[i][j]);
    }
  }

  return maxLength * maxLength;
}

// https://leetcode.com/problems/maximal-rectangle/description/
// 用maximalRectangleInHistogram的思想解

// 时间复杂度O(matrixRowSize * matrixColSize)
// 空间复杂度O(matrixColSize)

inline int max(int a, int b) { return a < b ? b : a; }

int maximalRectangleInHistogram(int *nums, int numsSize) {
  int stack[numsSize + 1];
  int top = 0;
  stack[top] = -1;
  int maxArea = 0;
  for (int i = 0; i < numsSize; i++) {
    while (top >= 0 && nums[i] < nums[stack[top]]) {
      int h = nums[stack[top]];
      top--;
      int w = i - 1 - stack[top];
      int area = h * w;
      maxArea = max(area, maxArea);
    }
    stack[++top] = i;
  }

  while (top > 0) {
    int h = nums[stack[top]];
    top--;
    int w = numsSize - 1 - stack[top];
    int area = h * w;
    maxArea = max(area, maxArea);
  }

  return maxArea;
}

int maximalRectangle(char **matrix, int matrixRowSize, int matrixColSize) {
  if (matrixRowSize == 0 || matrixColSize == 0) {
    return 0;
  }

  int maxArea = 0;
  int nums[matrixColSize];

  for (int i = 0; i < matrixRowSize; i++) {
    for (int j = 0; j < matrixColSize; j++) {
      int n = matrix[i][j] - '0';
      if (i == 0) {
        nums[j] = n;
      } else {
        nums[j] = n == 0 ? 0 : nums[j] + n;
      }
    }
    int area = maximalRectangleInHistogram(nums, matrixColSize);
    maxArea = max(area, maxArea);
  }

  return maxArea;
}

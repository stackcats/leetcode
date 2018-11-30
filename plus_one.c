// https://leetcode.com/problems/plus-one/description/

/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *plusOne(int *digits, int digitsSize, int *returnSize) {
  // 多开辟一个空间存进位
  *returnSize = digitsSize + 1;
  int *arr = (int *)malloc(sizeof(int) * (*returnSize));

  int j = 0;
  int c = 1;
  // 倒序存储结果
  for (int i = digitsSize - 1; i >= 0; i--) {
    int n = digits[i] + c;
    arr[j++] = n % 10;
    c = n / 10;
  }

  if (c == 1) {
    arr[j++] = 1;
  }

  *returnSize = j; // 重新赋值长度

  // 翻转结果
  for (int i = 0, j = *returnSize - 1; i < j; i++, j--) {
    int t = arr[i];
    arr[i] = arr[j];
    arr[j] = t;
  }

  return arr;
}

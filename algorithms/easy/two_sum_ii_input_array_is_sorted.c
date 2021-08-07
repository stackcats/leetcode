// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/description/

/**
 * Return an array of size *returnSize.
 * Note: The returned array must be malloced, assume caller calls free().
 */
int *twoSum(int *numbers, int numbersSize, int target, int *returnSize) {
  *returnSize = 2;
  int *res = (int *)malloc(sizeof(int) * 2);
  int i = 0;
  int j = numbersSize - 1;
  while (i < j) {
    int sum = numbers[i] + numbers[j];
    if (sum == target) {
      res[0] = i + 1;
      res[1] = j + 1;
      return res;
    } else if (sum < target) {
      i++;
    } else {
      j--;
    }
  }

  return res;
}

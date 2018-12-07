// https://leetcode.com/problems/first-missing-positive/description/

// 先排序然后求解
// 要求时间复杂度O(n)
// 正整数可以使用bucket sort实现时间复杂度为O(n)的排序

int firstMissingPositive(int *nums, int numsSize) {
  for (int i = 0; i < numsSize; i++) {
    if (nums[i] > 0 && nums[i] <= numsSize && nums[nums[i] - 1] != nums[i]) {
      int t = nums[nums[i] - 1];
      nums[nums[i] - 1] = nums[i];
      nums[i] = t;
      i--;
    }
  }

  for (int i = 0; i < numsSize; i++) {
    if (nums[i] != i + 1) {
      return i + 1;
    }
  }

  return numsSize + 1;
}

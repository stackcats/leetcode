// https://leetcode.com/problems/missing-number/description/

// 思路同First Missing Positive
// 处理边界略有不同
int missingNumber(int *nums, int numsSize) {
  for (int i = 0; i < numsSize; i++) {
    if (nums[i] < numsSize && nums[nums[i]] != nums[i]) {
      int t = nums[nums[i]];
      nums[nums[i]] = nums[i];
      nums[i] = t;
      i--;
    }
  }

  for (int i = 0; i < numsSize; i++) {
    if (nums[i] != i) {
      return i;
    }
  }

  return numsSize;
}

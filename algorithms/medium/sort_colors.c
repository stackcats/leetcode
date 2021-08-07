// https://leetcode.com/problems/sort-colors/description/

void sortColors(int *nums, int numsSize) {
  for (int i = 0; i < numsSize; i++) {
    if (nums[i] == 2) {
      for (int j = numsSize - 1; j > i; j--) {
        if (nums[j] < 2) {
          int t = nums[i];
          nums[i] = nums[j];
          nums[j] = t;
          break;
        }
      }
    }
    if (nums[i] == 1) {
      for (int j = numsSize - 1; j > i; j--) {
        if (nums[j] == 0) {
          int t = nums[i];
          nums[i] = nums[j];
          nums[j] = t;
        }
      }
    }
  }
}

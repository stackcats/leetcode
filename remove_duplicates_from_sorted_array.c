// https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
// 不需要向前拷贝所有元素

int removeDuplicates(int *nums, int numsSize) {
  if (!nums || numsSize == 0) {
    return 0;
  }
  int i = 0;
  int lastIndex = i;
  while (i < numsSize) {
    int j = i + 1;
    while (j < numsSize && nums[i] == nums[j]) {
      j++;
    }

    if (j < numsSize) {
      nums[++lastIndex] = nums[j];
    }

    i = j;
  }

  // O(n)
  return lastIndex + 1;
}

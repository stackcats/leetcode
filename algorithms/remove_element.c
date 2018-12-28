// https://leetcode.com/problems/remove-element/description/
// 利用类似选择排序的思想 将等于val的值置换到后面
int removeElement(int *nums, int numsSize, int val) {
  int i = 0;
  int j = numsSize - 1;
  while (i <= j) {
    while (i <= j && nums[i] != val) {
      i++;
    }

    while (i <= j && nums[j] == val) {
      j--;
    }

    if (i < j) {
      int t = nums[i];
      nums[i] = nums[j];
      nums[j] = t;
    }
  }

  return i;
}

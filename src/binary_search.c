// https://leetcode.com/problems/binary-search/description/

int search(int *nums, int numsSize, int target) {
  int i = 0;
  int j = numsSize - 1;
  while (i <= j) {
    int mid = i + (j - i) / 2; // 小心溢出
    if (nums[mid] == target) {
      return mid;
    } else if (nums[mid] < target) {
      i = mid + 1;
    } else {
      j = mid - 1;
    }
  }

  return -1;
}

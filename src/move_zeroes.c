// https://leetcode.com/problems/move-zeroes/description/

void moveZeroes(int *nums, int numsSize) {
  int ndx = 0;
  int i;
  for (i = 0; i < numsSize; ++i) {
    if (nums[i] != 0) {
      if (i > ndx) {
        int t = nums[ndx];
        nums[ndx] = nums[i];
        nums[i] = t;
      }
      ndx++;
    }
  }
}

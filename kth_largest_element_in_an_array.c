// https://leetcode.com/problems/kth-largest-element-in-an-array/description/

// 实现的有问题 并没有比直接qsort快
int qselect(int *nums, int left, int right) {
  int p = nums[left];
  int l = left;
  int r = right;
  while (l <= r) {
    if (nums[l] >= p) {
      l++;
      continue;
    }

    if (nums[r] <= p) {
      r--;
      continue;
    }

    int t = nums[l];
    nums[l] = nums[r];
    nums[r] = t;
  }

  nums[left] = nums[r];
  nums[r] = p;
  return r;
}

int findKthLargest(int *nums, int numsSize, int k) {
  int left = 0;
  int right = numsSize - 1;
  while (1) {
    int p = qselect(nums, left, right);
    if (p + 1 == k) {
      return nums[p];
    }
    if (p + 1 > k) {
      right = p - 1;
    } else {
      left = p + 1;
    }
  }
}

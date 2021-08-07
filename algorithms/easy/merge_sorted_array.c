// https://leetcode.com/problems/merge-sorted-array/description/

void merge(int *nums1, int m, int *nums2, int n) {
  int *tmp = (int *)malloc(sizeof(int) * (m + n));
  int i = 0;
  int j = 0;
  int k = 0;
  while (i < m && j < n) {
    if (nums1[i] < nums2[j]) {
      tmp[k++] = nums1[i++];
    } else {
      tmp[k++] = nums2[j++];
    }
  }

  while (i < m) {
    tmp[k++] = nums1[i++];
  }
  while (j < n) {
    tmp[k++] = nums2[j++];
  }

  i = 0;
  while (i < k) {
    nums1[i] = tmp[i];
    i++;
  }

  free(tmp);
}

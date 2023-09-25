class Solution {
  List<List<int>> mergeArrays(List<List<int>> nums1, List<List<int>> nums2) {
    int i = 0;
    int j = 0;
    List<List<int>> res = [];
    while (i < nums1.length && j < nums2.length) {
      if (nums1[i][0] == nums2[j][0]) {
        res.add([nums1[i][0], nums1[i][1] + nums2[j][1]]);
        i++;
        j++;
      } else if (nums1[i][0] < nums2[j][0]) {
        res.add(nums1[i]);
        i++;
      } else {
        res.add(nums2[j]);
        j++;
      }
    }

    while (i < nums1.length) {
      res.add(nums1[i]);
      i++;
    }

    while (j < nums2.length) {
      res.add(nums2[j]);
      j++;
    }

    return res;
  }
}

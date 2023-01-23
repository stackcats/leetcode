class Solution {
public:
  int getCommon(vector<int> &nums1, vector<int> &nums2) {
    int l = 0, r = 0;
    while (l < nums1.size() && r < nums2.size()) {
      if (nums1[l] == nums2[r])
        return nums1[l];
      if (nums1[l] < nums2[r]) {
        l++;
      } else {
        r++;
      }
    }
    return -1;
  }
};

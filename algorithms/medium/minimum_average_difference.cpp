class Solution {
public:
  int minimumAverageDifference(vector<int> &nums) {
    long long right = 0;
    for (auto n : nums) {
      right += n;
    }
    long long left = 0;
    int ans = 0;
    long long min_avg = LONG_LONG_MAX;
    for (int i = 0; i < nums.size(); ++i) {
      left += nums[i];
      right -= nums[i];
      long long left_part = left / (i + 1);
      long long right_part =
          i == nums.size() - 1 ? 0 : right / (nums.size() - i - 1);
      long long diff = abs(left_part - right_part);
      if (min_avg > diff) {
        ans = i;
        min_avg = diff;
      }
    }
    return ans;
  }
};

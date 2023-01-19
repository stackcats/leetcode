class Solution {
public:
  int subarraysDivByK(vector<int> &nums, int k) {
    vector<int> counter(k);
    counter[0] = 1;
    int ans = 0, prefix_sum = 0;
    for (auto n : nums) {
      prefix_sum += n;
      int m = (prefix_sum % k + k) % k;
      ans += counter[m];
      counter[m]++;
    }
    return ans;
  }
};

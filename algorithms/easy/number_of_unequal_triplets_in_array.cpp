class Solution {
public:
    int unequalTriplets(vector<int>& nums) {
      unordered_map<int, int>mp;
      for (auto n : nums) {
        mp[n]++;
      }
      int left = 0, right = nums.size();
      int ans = 0;
      for (auto [_n, ct] : mp) {
        right -= ct;
        ans += left * ct * right;
        left += ct;
      }
      return ans;
    }
};

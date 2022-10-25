class Solution {
public:
  int mostFrequentEven(vector<int>& nums) {
    unordered_map<int, int> mp;
    int ans = -1;
    int max_freq = -1;
    for (auto n : nums) {
      if (n % 2 == 1) continue;
      mp[n]++;
      if (mp[n] > max_freq) {
        max_freq = mp[n];
        ans = n;
      } else if (mp[n] == max_freq) {
        ans = min(n, ans);
      }
    }
    return ans;
  }
};

class Solution {
public:
  int similarPairs(vector<string> &words) {
    unordered_map<int, int> mp;
    for (auto &w : words) {
      int res = 0;
      for (auto c : w) {
        res |= 1 << (c - 'a');
      }
      mp[res]++;
    }
    int ans = 0;
    for (auto &it : mp) {
      ans += it.second * (it.second - 1) / 2;
    }
    return ans;
  }
};

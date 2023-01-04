class Solution {
public:
  int minimumRounds(vector<int> &tasks) {
    unordered_map<int, int> mp;
    for (auto t : tasks) {
      mp[t]++;
    }

    int ans{0};
    for (auto &[_k, v] : mp) {
      if (v == 1) {
        return -1;
      }
      ans += int(ceil(v / 3.0));
    }
    return ans;
  }
};

class Solution {
public:
  int minFlipsMonoIncr(string s) {
    vector<int> f0(s.size() + 1), f1(s.size() + 1);
    for (int i = 1; i <= s.size(); i++) {
      f0[i] = f0[i - 1] + (s[i - 1] == '1');
      int j = s.size() - i;
      f1[j] = f1[j + 1] + (s[j] == '0');
    }
    int ans = INT_MAX;
    for (int i = 0; i < f0.size(); i++) {
      ans = min(ans, f0[i] + f1[i]);
    }
    return ans;
  }
};

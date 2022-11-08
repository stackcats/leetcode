class Solution {
public:
    vector<int> findArray(vector<int>& pref) {
      int acc = 0;
      vector<int> ans;
      for (auto x : pref) {
        ans.push_back(acc ^ x);
        acc = x;
      }
      return ans;
    }
};

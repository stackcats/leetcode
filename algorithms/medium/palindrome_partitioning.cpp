class Solution {
public:
  vector<vector<string>> ans;
  vector<string> subs;
  vector<vector<string>> partition(string s) {
    vector<string> subs;
    dfs(s, 0);
    return ans;
  }
  void dfs(const string &s, int l) {
    if (l == s.size()) {
      ans.push_back(subs);
      return;
    }
    for (int r = l; r < s.size(); r++) {
      if (is_palindrome(s, l, r)) {
        subs.push_back(s.substr(l, r - l + 1));
        dfs(s, r + 1);
        subs.pop_back();
      }
    }
  }

  bool is_palindrome(const string &s, int l, int r) {
    while (l < r) {
      if (s[l++] != s[r--])
        return false;
    }
    return true;
  }
};

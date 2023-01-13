class Solution {
public:
  unordered_map<int, vector<int>> mp;
  string s;
  int ans = 1;
  int longestPath(vector<int> &parent, string s) {
    for (int i = 0; i < parent.size(); i++) {
      mp[parent[i]].push_back(i);
    }
    this->s = s;
    dfs(0);
    return ans;
  }
  int dfs(int root) {
    int m = 0, n = 0;
    for (auto child : mp[root]) {
      int ct = dfs(child);
      if (s[child] != s[root]) {
        if (ct > n) n = ct;
        if (n > m) swap(m, n);
      }
    }

    ans = max(ans, m + n + 1);
    return m + 1;
  }
};

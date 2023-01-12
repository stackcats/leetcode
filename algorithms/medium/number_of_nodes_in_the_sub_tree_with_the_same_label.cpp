class Solution {
public:
  unordered_map<int, vector<int>> mp;
  string labels;
  vector<int> ans;
  vector<int> countSubTrees(int n, vector<vector<int>> &edges, string labels) {
    ans.resize(n);
    for (auto &edge : edges) {
      mp[edge[0]].push_back(edge[1]);
      mp[edge[1]].push_back(edge[0]);
    }
    this->labels = labels;
    dfs(0, -1);
    return ans;
  }

  vector<int> dfs(int curr, int parent) {
    vector<int> ct(26);
    for (auto &child : mp[curr]) {
      if (child == parent)
        continue;
      auto s = dfs(child, curr);
      for (int i = 0; i < s.size(); i++) {
        ct[i] += s[i];
      }
    }
    int ndx = labels[curr] - 'a';
    ct[ndx]++;
    ans[curr] = ct[ndx];
    return ct;
  }
};

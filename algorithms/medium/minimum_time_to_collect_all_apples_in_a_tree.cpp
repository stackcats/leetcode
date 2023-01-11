class Solution {
public:
    int minTime(int n, vector<vector<int>>& edges, vector<bool>& hasApple) {
      unordered_map<int, vector<int>> mp;
      for (auto& edge : edges) {
        mp[edge[0]].push_back(edge[1]);
        mp[edge[1]].push_back(edge[0]);
      }
      return max(aux(mp, 0, hasApple, -1) - 2, 0);
    }

  int aux(unordered_map<int, vector<int>>& mp, int curr, vector<bool>& hasApple, int parent) {
    int ct = 0;
    for (auto child : mp[curr]) {
      if (child == parent) continue;
      ct += aux(mp, child, hasApple, curr);
    }
    if (ct > 0) return ct + 2;
    if (hasApple[curr]) return 2;
    return 0;
  }
};

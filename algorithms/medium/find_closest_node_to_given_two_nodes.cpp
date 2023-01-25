class Solution {
public:
  int closestMeetingNode(vector<int> &edges, int node1, int node2) {
    auto mp1 = dfs(edges, node1, 0);
    auto mp2 = dfs(edges, node2, 0);
    int ans = -1, min_dis = INT_MAX;
    for (int i = 0; i < edges.size(); i++) {
      auto f1 = mp1.find(i), f2 = mp2.find(i);
      if (f1 != mp1.end() && f2 != mp2.end()) {
        int ma = max(f1->second, f2->second);
        if (min_dis > ma) {
          ans = i;
          min_dis = ma;
        }
      }
    }
    return ans;
  }
  unordered_map<int, int> dfs(vector<int> &edges, int curr, int dis) {
    unordered_map<int, int> mp;
    while (curr != -1 && mp.find(curr) == mp.end()) {
      mp.insert({curr, dis++});
      curr = edges[curr];
    }
    return mp;
  }
};

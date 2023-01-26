class Solution {
public:
  int findCheapestPrice(int n, vector<vector<int>> &flights, int src, int dst,
                        int k) {
    unordered_map<int, vector<pair<int, int>>> graph;
    for (int i = 0; i < flights.size(); i++) {
      graph[flights[i][0]].push_back({flights[i][1], flights[i][2]});
    }

    queue<array<int, 3>> q;
    q.push({src, k, 0});
    vector<int> ans(n, -1);
    while (!q.empty()) {
      auto [curr, ct, price] = q.front();
      q.pop();
      if (ans[curr] > 0 && ans[curr] < price) {
        continue;
      }
      ans[curr] = price;
      if (ct < 0) {
        continue;
      }
      for (const auto &[d, p] : graph[curr]) {
        q.push({d, ct - 1, p + price});
      }
    }
    return ans[dst];
  }
};

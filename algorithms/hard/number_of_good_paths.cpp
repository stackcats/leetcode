class Solution {
public:
  vector<int> root;
  vector<int> rank;
  void init(int n) {
    root = vector<int>(n, -1);
    rank = vector<int>(n);
  }
  int find(int k) {
    return root[k] == -1 ? k : root[k] = find(root[k]);
  }
  void unite(int a, int b) {
    a = find(a), b = find(b);
    if (a == b) return;
    if (rank[a] < rank[b]) {
        root[a] = b;
    } else {
        root[b] = a;
        if (rank[a] == rank[b]) rank[a]++;
    }
  }
  int numberOfGoodPaths(vector<int> &vals, vector<vector<int>> &edges) {
    init(vals.size());
    unordered_map<int, vector<int>> valMp;
    for (int i = 0; i < vals.size(); i++) {
      valMp[vals[i]].push_back(i);
    }
    unordered_map<int, vector<pair<int, int>>> edgeMp;
    for (auto& edge : edges) {
      int a = edge[0], b = edge[1];
      if (vals[a] < vals[b]) {
        swap(a, b);
      }
      edgeMp[vals[a]].push_back({a, b});
    }
    int ans = 0;
    set<int>valSet(vals.begin(), vals.end());
    for (auto& v : valSet) {
      for (auto& [a, b] : edgeMp[v]) {
        unite(a, b);
      }
      unordered_map<int, int>counter;
      for (auto& ndx : valMp[v]) {
        counter[find(ndx)]++;
      }
      for (auto& [_, v] : counter) {
        ans += v * (v - 1) / 2;
      }
    }
    return ans + vals.size();
  }
};

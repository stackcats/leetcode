class Solution {
public:
  unordered_map<int, int> root;
  unordered_map<int, int> rank;
  int ct;
  int find(int k) {
    if (root.find(k) == root.end()) {
      ct++;
      return root[k] = k;
    }
    if (root[k] == k) return k;
    return root[k] = find(root[k]);
  }
  void unite(int a, int b) {
    a = find(a);
    b = find(b);
    if (a == b) return;
    ct--;
    if (rank[a] < rank[b]) {
      root[a] = b;
    } else {
      root[b] = a;
      if (rank[a] == rank[b]) rank[a]++;
    }
  }
  int removeStones(vector<vector<int>> &stones) {
    for (auto &stone : stones) {
      unite(stone[0], stone[1] + 10000);
    }

    return stones.size() - ct;
  }
};

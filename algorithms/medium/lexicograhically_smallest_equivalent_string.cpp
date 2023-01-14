class Solution {
public:
  array<int, 26> uf;
  string smallestEquivalentString(string s1, string s2, string baseStr) {
    for (int i = 0; i < uf.size(); i++) {
      uf[i] = i;
    }

    for (int i = 0; i < s1.size(); i++) {
      unite(s1[i] - 'a', s2[i] - 'a');
    }

    string ans;
    for (auto &c : baseStr) {
      ans.push_back(find(c - 'a') + 'a');
    }
    return ans;
  }

  void unite(int c1, int c2) {
    int r1 = find(c1);
    int r2 = find(c2);
    if (r1 == r2)
      return;
    if (r1 < r2)
      uf[r2] = r1;
    else
      uf[r1] = r2;
  }

  int find(int c) {
    int r = uf[c];
    if (r == c)
      return r;
    r = uf[c] = find(r);
    return r;
  }
};

class Solution {
public:
    int minAreaRect(vector<vector<int>>& points) {
      set<vector<int> > st;
      int ans = INT_MAX;
      for (auto& p1 : points) {
        for (auto& p2 : st) {
          int x1 = p1[0], y1 = p1[1], x2 = p2[0], y2 = p2[1];
          if (x1 == x2 || y1 == y2) continue;
          if (st.count({x1, y2}) && st.count({x2, y1})) {
            const int area = abs(x1 - x2) * abs(y1 - y2);
            ans = min(ans, area);
          }
        }
        st.insert(p1);
      }
      return ans == INT_MAX ? 0 : ans;
    }
};

class Solution {
public:
    int captureForts(vector<int>& forts) {
      int prev{0};
      int ans{0};
      for (int i = 0; i < forts.size(); i++) {
        if (forts[i] == 0) continue;
        if (forts[i] == -forts[prev]) {
          ans = max(ans, i - prev - 1);
        }
        prev = i;
      }
      return ans;
    }
};

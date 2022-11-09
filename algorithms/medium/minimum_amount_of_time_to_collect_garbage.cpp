class Solution {
public:
    int garbageCollection(vector<string>& garbage, vector<int>& travel) {
      int dis = 0;
      int lastG = 0, lastP = 0, lastM = 0;
      int ans = 0;
      for (int i = 0; i < garbage.size(); ++i) {
        if (i > 0) dis += travel[i-1];
        int g = 0, p = 0, m = 0;
        for (auto& c : garbage[i]) {
          if (c == 'G') g++;
          else if (c == 'P') p++;
          else m++;
        }
        if (g > 0) {
          ans += dis - lastG;
          lastG = dis;
        }
        if (p > 0) {
          ans += dis - lastP;
          lastP = dis;
        }
        if (m > 0) {
          ans += dis - lastM;
          lastM = dis;
        }
        ans += g + p + m;
      }
      return ans;
    }
};

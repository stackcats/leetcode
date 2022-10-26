class Solution {
public:
    int minimumRecolors(string blocks, int k) {
      int ans = 0;
      int i = 0;
      for (; i < min((int)blocks.size(), k); ++i) {
        if (blocks[i] == 'W') ans++;
      }
      if (i == blocks.size()) return ans;
      int curr = ans;
      for (; i < blocks.size(); ++i) {
        if (blocks[i-k] == 'W') curr--;
        if (blocks[i] == 'W') curr++;
        ans = min(curr, ans);
      }
      return ans;
    }
};

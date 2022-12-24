class Solution {
public:
    int numTilings(int n) {
      int a = 1, b = 2, c = 5;
      if (n == 1) return a;
      if (n == 2) return b;
      if (n == 3) return c;
      for (int i = 4; i <= n; i++) {
        int d = (c * 2 % 1000000007 + a) % 1000000007;
        a = b;
        b = c;
        c = d;
      }
      return c;
    }
};

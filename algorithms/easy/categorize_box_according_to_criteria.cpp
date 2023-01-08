class Solution {
public:
    string categorizeBox(int length, int width, int height, int mass) {
      bool is_bulky = length >= 1e4 || width >= 1e4 || height >= 1e4 || (long)length * width * height >= 1e9;
      bool is_heavy = mass >= 100;
      if (is_bulky && is_heavy) return "Both";
      if (is_bulky) return "Bulky";
      if (is_heavy) return "Heavy";
      return "Neither";
    }
};

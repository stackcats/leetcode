class Solution {
public:
    int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
      vector<int> diff(gas.size(), 0);
      int total{0};
      for (int i = 0; i < gas.size(); i++) {
        diff[i] = gas[i] - cost[i];
        total += diff[i];
      }
      if (total < 0) return -1;
      int ans{0}, g{0};
      for (int i = 0; i < diff.size(); i++) {
        if (g < 0) {
          ans = i;
          g = 0;
        }
        g += diff[i];
      }
      return ans;
    }
};

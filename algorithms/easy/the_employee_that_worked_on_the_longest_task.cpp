class Solution {
public:
    int hardestWorker(int n, vector<vector<int>>& logs) {
      int preEnd = 0;
      int ans = 0;
      int longest = 0;
      for (int i = 0; i < logs.size(); ++i) {
        int duration = logs[i][1] - preEnd;
        if (duration > longest) {
          longest = logs[i][1] - preEnd;
          ans = logs[i][0];
        } else if (duration == longest) {
          ans = min(ans, logs[i][0]);
        }
        preEnd = logs[i][1];
      }
      return ans;
    }
};

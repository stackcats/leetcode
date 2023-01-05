class Solution {
public:
  vector<int> sequentialDigits(int low, int high) {
    priority_queue<int, vector<int>, greater<int> > pq;
    for (int i = 1; i < 10; i++) {
      int n = i;
      while (n < low && n % 10 != 0) {
        n = n * 10 + n % 10 + 1;
      }
      while (n <= high && n % 10 != 0) {
        pq.push(n);
        n = n * 10 + n % 10 + 1;
      }
    }
    vector<int> ans;
    while (!pq.empty()) {
      ans.push_back(pq.top());
      pq.pop();
    }
    return ans;
  }
};

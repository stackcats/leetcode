class Solution {
public:
  unordered_set<int> st;
  int distinctPrimeFactors(vector<int> &nums) {
    for (auto n : nums) {
      primeFactor(n);
    }
    return st.size();
  }
  void primeFactor(int n) {
    int p = 2;
    while (n >= p) {
      if (n % p == 0) {
        st.insert(p);
        n /= p;
      } else {
        p++;
      }
    }
  }
};

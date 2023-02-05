class Solution {
public:
  vector<int> separateDigits(vector<int> &nums) {
    vector<int> ans;
    for (auto n : nums) {
      vector<int> arr;
      while (n > 0) {
        arr.push_back(n % 10);
        n /= 10;
      }
      ans.insert(ans.end(), arr.rbegin(), arr.rend());
    }
    return ans;
  }
};

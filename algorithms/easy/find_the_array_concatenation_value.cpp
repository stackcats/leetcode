class Solution {
public:
  long long findTheArrayConcVal(vector<int> &nums) {
    long long ans = 0;
    for (int i = 0, j = nums.size() - 1; i <= j; i++, j--) {
      string s = to_string(nums[i]);
      if (i < j)
        s += to_string(nums[j]);
      ans += stoll(s, nullptr, 10);
    }
    return ans;
  }
};

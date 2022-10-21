class Solution {
public:
    vector<int> answerQueries(vector<int>& nums, vector<int>& queries) {
      sort(nums.begin(), nums.end());
      int sum = 0;
      for (int i = 0; i < nums.size(); i++) {
        sum += nums[i];
        nums[i] = sum;
      }
      vector<int> ans;
      for (auto q : queries) {
        auto n = upper_bound(nums.begin(), nums.end(), q);
        ans.push_back(distance(nums.begin(), n));
      }
      return ans;
    }
};

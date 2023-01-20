class Solution {
public:
  vector<vector<int>> ans;
  vector<vector<int>> findSubsequences(vector<int> &nums) {
    vector<int> st;
    dfs(nums, 0, st);
    return ans;
  }
  void dfs(vector<int> &nums, int curr, vector<int> &st) {
    if (st.size() > 1) {
      ans.push_back(st);
    }
    unordered_set<int> mp;
    for (int i = curr; i < nums.size(); i++) {
      if (st.size() > 0 && nums[i] < st[st.size() - 1] || mp.find(nums[i]) != mp.end()) {
        continue;
      }
      mp.insert(nums[i]);
      st.push_back(nums[i]);
      dfs(nums, i + 1, st);
      st.pop_back();
    }
  }
};

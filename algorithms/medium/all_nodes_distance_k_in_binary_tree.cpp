/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
  vector<int> distanceK(TreeNode *root, TreeNode *target, int k) {
    dfs(root, nullptr);
    return bfs(target->val, k);
  }
  vector<int> bfs(int val, int k) {
    vector<int> ans, next_level;
    ans.push_back(val);
    unordered_set<int> visited;
    visited.insert(val);
    while (k-- > 0) {
      for (auto n : ans) {
        for (auto m : mp[n]) {
          if (visited.count(m))
            continue;
          visited.insert(m);
          next_level.push_back(m);
        }
      }
      ans.clear();
      swap(ans, next_level);
    }
    return ans;
  }
  void dfs(TreeNode *p, TreeNode *root) {
    if (p == nullptr)
      return;
    if (root) {
      mp[root->val].push_back(p->val);
      mp[p->val].push_back(root->val);
    }
    if (p->left)
      dfs(p->left, p);
    if (p->right)
      dfs(p->right, p);
  }

private:
  unordered_map<int, vector<int>> mp;
};

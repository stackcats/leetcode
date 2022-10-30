class Solution {
public:
    string oddString(vector<string>& words) {
      for (int j = 1; j < words[0].size(); ++j) {
        unordered_map<int, vector<int>> mp;
        for (int i = 0; i < words.size(); ++i) {
          mp[words[i][j] - words[i][j-1]].push_back(i);
        }
        if (mp.size() != 1) {
          for (auto& x : mp) {
            if (x.second.size() == 1) {
              return words[x.second[0]];
            }
          }
        }
      }
      return "";
    }
};

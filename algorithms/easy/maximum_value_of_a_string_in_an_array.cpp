class Solution {
public:
    int maximumValue(vector<string>& strs) {
      int ans = 0;
      for (auto& s : strs) {
        int i = 0;
        try {
          size_t pos{};
          i = stoi(s, &pos);
          if (pos != s.size()) i = s.size();
        } catch(invalid_argument const& ex) {
          i = s.size();
        }
        ans = max(ans, i);
      }
      return ans;
    }
};

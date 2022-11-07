typedef unordered_map<string, int>::value_type map_value_type;

class Solution {
public:
  int longestPalindrome(vector<string> &words) {
    unordered_map<string, int> mp;
    int ct = 0;
    for (auto &w : words) {
      auto rw = string(rbegin(w), rend(w));
      if (mp[rw] > 0) {
        ct += 4;
        mp[rw]--;
      } else {
        mp[w]++;
      }
    }

    if (mp.end() != find_if(mp.begin(), mp.end(), [](const map_value_type &vt) {
          return vt.second > 0 &&
                 string(rbegin(vt.first), rend(vt.first)) == vt.first;
        })) {
      ct += 2;
    }
    return ct;
  }
};

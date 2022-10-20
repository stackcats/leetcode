class Solution {
public:
    bool checkDistances(string s, vector<int>& distance) {
      int mp[26] = {};
      for (int i = 0; i < s.size(); i++) {
        const int ndx = s[i] - 'a';
        if (mp[ndx] > 0 && distance[ndx] != i - mp[ndx]) return false;
        mp[ndx] = i + 1;
      }
      return true;
    }
};

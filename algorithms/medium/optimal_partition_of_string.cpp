class Solution {
public:
  int partitionString(string s) {
    int ct = 0;
    unordered_set<char>set;
    for (auto& c : s) {
      if (set.find(c) != set.end()) {
        ct++;
        set.clear();
      }
      set.insert(c);
    }
    return ct + 1;
  }
};

bool isEqual(vector<int> v) {
  vector<int> erased;
  copy_if(begin(v), end(v), back_inserter(erased), [](int x) { return x != 0; });
  set<int> s(erased.begin(), erased.end());
  return s.size() == 1;
}

class Solution {
public:
  bool equalFrequency(string word) {
    vector<int> map(26, 0);
      
    for (auto c : word) {
      map[c-'a']++;
    }
    for (int i = 0; i < 26; i++) {
      if (map[i] == 0) continue;
      map[i]--;
      if (isEqual(map)) {
        return true;
      }
      map[i]++;
    }
    return false;
  }
};

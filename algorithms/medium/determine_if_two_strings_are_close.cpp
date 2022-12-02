class Solution {
public:
    bool closeStrings(string word1, string word2) {
      if (word1.size() != word2.size()) return false;
      array<int, 26> a1{0}, a2{0};
      unordered_set<int> s1, s2;
      for (int i = 0; i < word1.size(); ++i) {
        char c1 = word1[i], c2 = word2[i];
        a1[c1-'a']++;
        a2[c2-'a']++;
        s1.insert(c1);
        s2.insert(c2);
      };

      sort(a1.begin(), a1.end());
      sort(a2.begin(), a2.end());
      return a1 == a2 && s1 == s2;
    }
};

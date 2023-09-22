class Solution {
  String removeStars(String s) {
    List<String> ans = [];
    for (int i = 0; i < s.length; i++) {
      if (s[i] == "*") {
        ans.removeLast();
      } else {
        ans.add(s[i]);
      }
    }
    return ans.join("");
  }
}

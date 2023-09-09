class Solution {
  int vowelStrings(List<String> words, int left, int right) {
    RegExp exp = RegExp(r'^[aeiou](.*[aeiou])?$');
    int ct = 0;
    for (String w in words.sublist(left, right + 1)) {
      if (exp.hasMatch(w)) {
        ct += 1;
      }
    }
    return ct;
  }
}

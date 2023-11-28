class Solution {
  List<int> findWordsContaining(List<String> words, String x) {
    final arr = <int>[];
    for (int i = 0; i < words.length; i++) {
        if (words[i].contains(x)) {
            arr.add(i);
        }
    }
    return arr;
  }
}

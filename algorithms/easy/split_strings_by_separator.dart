class Solution {
  List<String> splitWordsBySeparator(List<String> words, String separator) {
    return words
        .expand((word) => word.split(separator).where((w) => w.isNotEmpty))
        .toList();
  }
}

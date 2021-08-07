/**
 * @param {string} paragraph
 * @param {string[]} banned
 * @return {string}
 */
var mostCommonWord = function(paragraph, banned) {
  paragraph = paragraph.replace(/[!?',;.]/g, ' ').split(' ');
  const dt = {};
  let ct = 0;
  let ans = '';
  for (const each of paragraph) {
    if (each === '') {
      continue;
    }
    const word = each.toLowerCase();
    if (banned.includes(word)) {
      continue;
    }
    if (word in dt) {
      dt[word] += 1;
    } else {
      dt[word] = 1;
    }
    if (ct < dt[word]) {
      ct = dt[word];
      ans = word;
    }
  }
  return ans;
};

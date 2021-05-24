import re

def h(x):
    return x[0]

class Solution:
    def sortSentence(self, s: str) -> str:
        lst = []
        arr = s.split(' ')
        for s in arr:
            n = int(re.sub('[^0-9]+', '', s))
            t = re.sub('\d', '', s)
            lst.append((n, t))

        lst.sort(key=h)
        return " ".join([x[1] for x in lst])

class Solution(object):
    def countCharacters(self, words, chars):
        """
        :type words: List[str]
        :type chars: str
        :rtype: int
        """
        charsDt = self.sToD(chars)
        ans = 0
        for w in words:
            d = self.sToD(w)
            canFormed = True
            for k in d:
                if k not in charsDt:
                    canFormed = False
                    break
                if d[k] > charsDt[k]:
                    canFormed = False
                    break
            if canFormed:
                ans += len(w)
        return ans
    def sToD(self, s):
        d = {}
        for c in s:
            if c in d:
                d[c] += 1
            else:
                d[c] = 1
        return d

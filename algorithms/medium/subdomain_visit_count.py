def helper(s):
    ans = []
    i = 0
    while i < len(s):
        ans.append(s[i:])
        while i < len(s) and s[i] != '.':
            i += 1

        i += 1
        
    return ans

class Solution:
    def subdomainVisits(self, cpdomains):
        """
        :type cpdomains: List[str]
        :rtype: List[str]
        """
        dt = {}
        for x in cpdomains:
            [ct, domain] = x.split(' ')
            ct = int(ct)
            ds = helper(domain)
            for d in ds:
                if d not in dt:
                    dt[d] = ct
                else:
                    dt[d] += ct
                    
        ans = []
        for k, v in dt.items():
            ans.append("%d %s" % (v, k))
        
        return ans

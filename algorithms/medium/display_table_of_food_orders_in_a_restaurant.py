from collections import defaultdict

class Solution:
    def displayTable(self, orders: List[List[str]]) -> List[List[str]]:
        counter = defaultdict(int)
        foods = set()
        tables = set()
        for [_, t, f] in orders:
            tables.add(t)
            foods.add(f)
            counter[(t, f)] += 1

        tables = list(tables)
        tables.sort(key=int)
        foods = list(foods)
        foods.sort()
        header = ['Table'] + foods
        ans = [header]
        for t in tables:
            row = [t]
            for f in foods:
                n = counter[(t, f)]
                row.append(str(n))
            ans.append(row)
        return ans
        

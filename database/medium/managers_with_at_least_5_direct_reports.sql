--https://leetcode.com/problems/managers-with-at-least-5-direct-reports/?envType=study-plan-v2&envId=top-sql-50

SELECT E1.name
FROM Employee E1
INNER JOIN (SELECT managerId FROM Employee GROUP BY managerId HAVING COUNT(managerId) >= 5) E2
ON E1.id = E2.managerId;
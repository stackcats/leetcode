# Write your MySQL query statement below

SELECT IFNULL(
       (SELECT Salary 
       FROM Employee
       GROUP BY Salary
       ORDER BY Salary DESC
       LIMIT 1
       OFFSET 1),
       null)
AS SecondHighestSalary;

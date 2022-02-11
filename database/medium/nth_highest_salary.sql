CREATE FUNCTION getNthHighestSalary(N INT) RETURNS INT
BEGIN
RETURN
(
    WITH cte AS
             (SELECT salary,
                     dense_rank() over (ORDER BY salary DESC) AS `rank`
              FROM employee)
    SELECT salary
    FROM cte
    WHERE `rank` = n
    LIMIT 1
);
END

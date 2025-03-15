WITH RECURSIVE
    LevelCTE AS (SELECT employee_id, manager_id, 1 AS level
                 FROM Employees
                 WHERE manager_id IS NULL
                 UNION ALL
                 SELECT e.employee_id, e.manager_id, lc.level + 1
                 FROM Employees e
                          JOIN LevelCTE lc ON e.manager_id = lc.employee_id),
    RecursiveTeams AS (SELECT employee_id, manager_id, employee_id AS root_manager
                       FROM Employees
                       UNION ALL
                       SELECT e.employee_id, e.manager_id, rt.root_manager
                       FROM Employees e
                                JOIN RecursiveTeams rt ON e.manager_id = rt.employee_id),
    BudgetCTE AS (SELECT rt.root_manager AS employee_id, SUM(e.salary) AS budget
                  FROM RecursiveTeams rt
                           JOIN Employees e ON rt.employee_id = e.employee_id
                  GROUP BY rt.root_manager),
    TeamSizeCTE AS (SELECT root_manager employee_id, COUNT(employee_id) - 1 AS team_size
                    FROM RecursiveTeams
                    GROUP BY root_manager)

SELECT e.employee_id,
       e.employee_name,
       COALESCE(lc.level, 1)         level,
       COALESCE(tc.team_size, 0)     team_size,
       COALESCE(bc.budget, e.salary) budget
FROM Employees e
         LEFT JOIN LevelCTE lc ON e.employee_id = lc.employee_id
         LEFT JOIN BudgetCTE bc ON e.employee_id = bc.employee_id
         LEFT JOIN TeamSizeCTE tc ON e.employee_id = tc.employee_id
ORDER BY level, budget DESC, employee_name;

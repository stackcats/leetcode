# Write your MySQL query statement below
select employee_id, case when name not like 'M%' and mod(employee_id, 2) = 1 then salary else 0 end as bonus
from Employees

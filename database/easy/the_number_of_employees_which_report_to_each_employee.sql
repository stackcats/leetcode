with cte as (
    select 
    reports_to, 
    name, 
    count(1) as reports_count,
    sum(age) as total_age
    from Employees
    where reports_to is not null
    group by reports_to
)
select 
t.employee_id,
t.name,
cte.reports_count,
round(cte.total_age / cte.reports_count) as average_age
from cte
left join Employees t on t.employee_id = cte.reports_to
order by t.employee_id


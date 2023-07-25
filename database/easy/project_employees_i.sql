select p.project_id,
round(sum(e.experience_years) / count(*), 2) as average_years
from Project p
left join Employee e on p.employee_id = e.employee_id
group by p.project_id

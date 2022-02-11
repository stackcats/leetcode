# Write your MySQL query statement below
select
id,
case when row_number() over (order by id) % 2 = 1
then lead(student, 1, student) over (order by id)
else lag(student) over (order by id) end as student
from Seat order by id;

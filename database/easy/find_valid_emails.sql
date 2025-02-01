-- Write your PostgreSQL query statement below
select 
user_id, email
from Users
where 
email ~* '(\w|_)*@[a-z]*\.com'
order by user_id

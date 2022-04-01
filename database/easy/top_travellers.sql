# Write your MySQL query statement below
select u.name, coalesce(sum(r.distance), 0) as travelled_distance
from Users u
left join Rides r on u.id = r.user_id
group by r.user_id
order by sum(r.distance) desc, u.name

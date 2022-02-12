# Write your MySQL query statement below

select request_at Day,
                  round(avg(case
                                when t.status != 'completed' then 1
                                else 0
                            end), 2) `Cancellation Rate`
from trips t
where client_id not in
    (select users_id
     from users
     where banned = 'Yes')
  and driver_id not in
    (select users_id
     from users
     where banned = 'Yes')
  and request_at between '2013-10-01' and '2013-10-03'
group by t.request_at
order by t.request_at

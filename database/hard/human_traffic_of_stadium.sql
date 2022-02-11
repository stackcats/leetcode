# Write your MySQL query statement below
with cte(startId, endId) as
  (SELECT L1.id,
          L3.id
   FROM Stadium L1,
        Stadium L2,
        Stadium L3
   WHERE L3.id = L2.id + 1
     AND L2.id = L1.id + 1
     AND L1.people >= 100
     AND L2.people >= 100
     AND L3.people >= 100)
select distinct id,
                visit_date,
                people
from Stadium,
     cte
where id between startId and endId

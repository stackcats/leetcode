# Write your MySQL query statement below

  (SELECT L1.id,
          L1.visit_date,
          L1.people
   FROM Stadium L1,
        Stadium L2,
        Stadium L3
   WHERE L3.id = L2.id + 1
     AND L2.id = L1.id + 1
     AND L1.people >= 100
     AND L2.people >= 100
     AND L3.people >= 100)
union
  (SELECT L2.id,
          L2.visit_date,
          L2.people
   FROM Stadium L1,
        Stadium L2,
        Stadium L3
   WHERE L3.id = L2.id + 1
     AND L2.id = L1.id + 1
     AND L1.people >= 100
     AND L2.people >= 100
     AND L3.people >= 100)
union
  (SELECT L3.id,
          L3.visit_date,
          L3.people
   FROM Stadium L1,
        Stadium L2,
        Stadium L3
   WHERE L3.id = L2.id + 1
     AND L2.id = L1.id + 1
     AND L1.people >= 100
     AND L2.people >= 100
     AND L3.people >= 100)
order by id

-- Write your PostgreSQL query statement below
SELECT c1.patient_id,
       patient_name,
       age,
       MIN(c2.test_date) - MIN(c1.test_date) recovery_time
FROM covid_tests c1
         JOIN covid_tests c2
              ON c1.patient_id = c2.patient_id
                  AND c1.test_date < c2.test_date
                  AND c1.result = 'Positive'
                  AND c2.result = 'Negative'
         JOIN patients p ON c1.patient_id = p.patient_id
GROUP BY c1.patient_id, patient_name, age
ORDER BY recovery_time, patient_name


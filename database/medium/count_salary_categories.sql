-- Write your PostgreSQL query statement below
SELECT UNNEST(ARRAY ['High Salary', 'Low Salary', 'Average Salary']) category,
       UNNEST(ARRAY [
        COUNT(1) FILTER (WHERE income > 50000), 
        COUNT(1) FILTER (WHERE income < 20000) ,
        COUNT(1) FILTER (WHERE income BETWEEN 20000 AND 50000)
       ])                                                            accounts_count
FROM accounts

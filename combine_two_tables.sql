-- https://leetcode.com/problems/combine-two-tables/description/

SELECT FirstName, LastName, City, State 
FROM Person p
LEFT JOIN Address a ON p.PersonId = a.PersonId;

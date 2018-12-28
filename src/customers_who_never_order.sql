SELECT C.name Customers
FROM Customers C
LEFT JOIN Orders O ON C.id = O.CustomerId
WHERE O.CustomerId IS NULL

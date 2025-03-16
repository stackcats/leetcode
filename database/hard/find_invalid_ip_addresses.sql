SELECT ip,
       COUNT(1) invalid_count
FROM logs
WHERE ip !~ '^((25[0-5]|(2[0-4]|1[0-9]|[1-9]|)[0-9])(\.(?!$)|$)){4}$'
GROUP BY ip
ORDER BY 2 DESC, 1 DESC


```postgresql
SELECT substring (email FROM '[0-9]+')
	FROM em WHERE email ~ '[0-9]';

```
We are only matching the numbers

| substring |
| :-------: |
|    79     |
|     1     |

```postgresql
SELECT DISTINCT(substring(email FROM '.+@(.*)$')) FROM em;
```
Even though we are matching anything that contains `@`, we are only selecting whatever is after `@` since we are using `(.*)`

| substring |
| :-------: |
| apple.com |
| uiuc.edu  |
| umuc.edu  |
| umich.edu |
## Mixing grouping and regex
```postgresql
SELECT substring(email FROM '.+@(.*)$'), COUNT(substring(email FROM '.+@(.*)$')) 
	FROM em GROUP BY substring(email FROM '.+@(.*)$');
```

| substring | count |
| :-------- | :---: |
| apple.com |   2   |
| uiuc.edu  |   1   |
| umuc.edu  |   1   |
| umich.edu |   2   |

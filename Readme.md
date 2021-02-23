# Rust Dynamic Programming: The Long Trip

You are going on a long trip. You start on the road at mile post 0. Along the way there are n hotels, at mile posts *a1 < a2 < ... < an*, where each ai is measured from the starting point. The only places you are allowed to stop are at these hotels, but you can choose which of the hotels you stop at. You must stop at the final hotel (at distance an) which is your destination. 

You'd ideally like to travel 200 miles a day, but this may not be possible (depending on the spacing of hotels). If you travel x miles during a day, the penalty for that day is (200-x)^2. You want to plan your trip so as to minimize the total penalty. That is, the sum over all travel days of the daily penalties.

## Give an efficient algorithm that determines the optimal sequence of  hotels at which to stop.

### Subproblem
Let H be an array of hotels visited with (H[0] = a1 .... H[n] = an) to get the optimal sequence of hotels to stop at. 

### Recurrence
Start at hotel i and loop through each possible hotel to travel to beyond it. For each iteration, find the minimum penalty. When found, the minimum j will be added to the stop H[s]. That stop will become hotel i and the iteration continues until it reaches the last stop at n. 

``` 
while i < n, j = i + i -> n, s = 0 :
    i = H[s] = min j {(200 - (aj-ai))^2} & s += 1 
```

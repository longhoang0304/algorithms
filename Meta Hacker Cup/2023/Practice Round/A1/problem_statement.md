This problem shares some similarities with A2, with key differences in bold.

Problem solving skills are applicable to many daily musings. For instance, you might ponder over shared birthdays, bird houses, mapmaking, or ordering an exact number of chicken nuggets. Naturally, another great question to ponder is: how many deckers of a cheeseburger you could build if you spent your entire salary on fast food!
Specifically, you're interested in building a K-decker cheeseburger, which alternates between buns, cheese, and patty starting and ending with a bun. You've already bought S single cheeseburgers and D double cheeseburgers. Each provides you with two buns, though a single provides one patty and one cheese, while a double provides two patties and two cheese.

You'd like to know whether you can build a K-decker cheeseburger with the ingredients from S single and D double cheeseburgers.

### Constraints
- 1≤T≤40
- 0≤S,D≤100
- 1≤K≤100

### Input Format
- Input begins with an integer TT, the number of test cases. Each case contains one line with three space-separated integers, S and D and K.

### Output Format
- For the ith test case, print "Case #i: " followed by "YES" if you have enough ingredients to build a K-decker cheeseburger, or "NO" otherwise.

### Sample Explanation

- In the first case, you have one single and one double cheeseburger. In total, you have 44 buns, 33 slices of cheese, and 33 patties. This gives you exactly enough ingredients to build a 33-decker cheeseburger.
In the second case, you have 44 buns, but a 44-decker cheeseburger would require 55, so you cannot build it.
In the third case, you have plenty of ingredients to build a 11-decker cheeseburger. You'll even have 44 single and 55 double cheeseburgers left over afterwards.

### Sample Input

```
7
1 1 3
0 2 4
5 5 1
0 1 1
1 1 2
97 1 99
97 1 100
```

### Sample Output

```
Case #1: YES
Case #2: NO
Case #3: YES
Case #4: YES
Case #5: YES
Case #6: YES
Case #7: NO
```
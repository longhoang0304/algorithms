## Vietnamese

### Url
https://cses.fi/problemset/task/1666

### Desc
Cho 1 đồ thị biểu diễn M đường đi giữa N thành phố với nhau. Tuy nhiên có vài thành phố bị cô lập do lũ làm hư đường. Hãy đếm xem cần bao nhiu con đường để tất cả thành phố có thể liên lạc được với nhau.

### Input
- Dòng đầu gồm 2 số N và M
- M là 2 số a và b, chỉ rằng thành phố a có thể liên lạc được với thành phố b

### Output
- Dòng đầu in ra số k - số đường cần xây
- Dòng tiếp theo gồm k dòng, mỗi dòng ghi ra con đường cần xây
- Có nhiều đáp án, chỉ cần đưa ra 1 đáp án đúng

### Giới hạn
- 1 ≤ n ≤ 10^5
- 1 ≤ m ≤ 2⋅10^5
- 1 ≤ a,b ≤n

### Sample Input
```
4 2
1 2
3 4
```

### Sample Output
```
1
2 3
```

### Giải thích
Cần xây 1 đường, từ thành phố 2 sang thành phố 3.

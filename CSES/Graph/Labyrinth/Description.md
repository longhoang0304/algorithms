## Vietnamese

### Url
https://cses.fi/problemset/task/1193

### Desc

Cho 1 ma trận NxM bao gồm 2 thông tin là tường - ký hiệu bởi # và sàn nhà, ký hiệu bởi . ! Cho 2 điểm A và B, tìm đường đi ngắn nhất từ A tới B.

### Input
- Dòng đầu gồm 2 số N và M
- N dòng tiếp theo gồm M ký tự # hoặc . biểu diễn tường hoặc sàn nhà cùng với 2 điểm A và B

### Output
- Dòng đầu in YES/NO. Yes nếu A có thể di tới B
- Dòng tiếp theo in chiều dài đường đi ngắn nhất từ A tới B
- Dòng cuối in cách đi từ A tới B bằng 4 ký tự LRUD! L = Left, R = Right, U = Up, D = Down

### Giới hạn
- 1 ≤ n,m ≤1000

### Sample Input
```
5 8
########
#.A#...#
#.##.#B#
#......#
########
```

### Sample Output
```
YES
9
LDDRRRRRU
```

### Giải thích
Có 3 phòng trong ma trận trên

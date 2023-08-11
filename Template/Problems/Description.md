## Vietnamese

### Url
https://onlinejudge.org/index.php?option=com_onlinejudge&Itemid=8&page=show_problem&problem=100

### Desc

Cho dãy số f được định nghĩa
f(n) = 3n + 1 nếu n là lẻ
f(n) = n / 2 nếu n là chẵn
f(n) = 1 nếu n = 1

Ví dụ:
n = 22 ta sẽ có dãy số
22 11 34 17 52 26 13 40 20 10 5 16 8 4 2 1

Dãy số này có 1 chu kỳ để trở về 1. Hãy tìm chu ký của dãy số đó

### Input
Các cặp i và j
0 < i, j < 10^4

### Output
Với mỗi năm i và j, in ra i, j và chu ký lớn nhất của số trong đoạn [i, j]

### Sample Input
```
1 10
100 200
201 210
900 1000
```

### Sample Output
```
1 10 20
100 200 125
201 210 89
900 1000 174
```

### Giải thích
Ở cặp 1, 10. Chu kỳ lớn nhất của các số từ 1 -> 10 là 20
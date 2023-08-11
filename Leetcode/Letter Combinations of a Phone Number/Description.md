## Vietnamese

### Url
https://leetcode.com/problems/letter-combinations-of-a-phone-number/

### Desc
Cho 1 chuỗi tối da 4 ký tự chứa các số từ 2 -> 9. In ra các tổ hợp ký tự có thể cấu tạo được từ dãy số trên. Các ký tự tương ứng với một số là các ký tự trên bàn phím numbad của điện thoại cũ.

```
2: ['a', 'b', 'c'],
3: ['d', 'e', 'f'],
4: ['g', 'h', 'i'],
5: ['j', 'k', 'l'],
6: ['m', 'n', 'o'],
7: ['p', 'q', 'r', 's'],
8: ['t', 'u', 'v'],
9: ['w', 'x', 'y', 'z'],
```

### Input
Một chuỗi S tối đa 4 ký tự. Chỉ gồm các số từ 2 -> 9

### Output
Mảng chứa toàn bộ các ký tự có thể hợp thành

### Sample Input
```
S = "23"
```

### Sample Output
```
["ad","ae","af","bd","be","bf","cd","ce","cf"]
```

### Giải thích
Bán phím số 2 có 3 ký tự [a, b, c]. Kết hợp với số 3 ta sẽ được ad, ae, af, ...

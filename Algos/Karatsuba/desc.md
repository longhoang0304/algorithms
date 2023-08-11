# Karatsuba

### Complexity
$O(n^{log_2{3}})$


### Mô tả
Cho số X và Y lần lượt gồm n và m chữ số. Tách 2 số này ra thành 4 phần a, b, c, d
- Ta có a = nửa phần đầu của X, b = nửa phần sau của X
- c = nửa phần đầu của Y, d = nửa phần sau của Y
- Ví dụ: X = 1234, Y = 5678 -> a = 12, b = 34, c = 56, d = 78

Khi này ta có
$X = 10^{\frac{n}{2}} \times a + b$
$Y = 10^{\frac{n}{2}} \times c + d$
$\rightarrow X \times Y = (10^{\frac{n}{2}} \times a + b) \times (10^{\frac{n}{2}} \times c + d)$
$= 10^n \times ac + 10^{\frac{n}{2}} \times (ad + bc) + db $

Vì a, b, c, d đã nhỏ hơn số X và Y ban đầu. Khi này ac, ad, bc, db thành các bài toán con của X * Y. Ta có thể dùng đệ quy để tính toán 4 thành phần này.

Tuy nhiên, ad + bc có thể gom thành (a + b)(c + d) thì khi này sẽ giúp tối giản được thêm 1 phép nhân. Sau khi tính xong, chỉ cần trừ ac và db là có được ad + bc!

Vậy khi này 3 bước để tính X * Y
- Tính a * c (1)
- Tính d * b (2)
- Tính (a + b)*(c + d) = ac + ad + bc + bd (3)
- (3) - (1) - (2) = ac + ad + bc + bd - ac - bd = ad + bc
- Áp dụng công thức ở trên

### Pseudocode
```

```
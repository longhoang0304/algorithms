# Merge Sort

### Complexity
$O(n logn)$

### Mô tả
- Cho 1 mảng Arr chưa được sắp xếp gồm n phần tử. Nhiệm vụ của mình là sắp xếp mảng này
  - Vd: Arr = [4, 2, 3, 1] -> Out = [1, 2, 3, 4]
- Đây là một bài toán chia để trị. THế nên để sắp xếp mảng này. Đầu tiên ta chia mảng nhỏ hơn thành 2 phần bằng nhau có chiều dài là n/2.
- Khi có 2 mảng nhỏ hơn, ta sẽ sắp xếp dựa trên 2 mảng này
- Sau khi có 2 mảng nhỏ hơn được sắp xếp, ta sẽ tiến hành gộp lại thành 1 mảng lớn
- 
// O(3n) -> O(n)
void moveZeroes(int* nums, int n){
    int i = 0;
    int j = 0;
    int z = 0;
    for (; i < n; i++) {
        if (!nums[i]) z++;
    }
    for (i = 0; i < n; i++) {
        if (nums[i]) nums[j++] = nums[i];
    }
    for (i = n - z; i < n; i++) {
        nums[i] = 0;
    }
}
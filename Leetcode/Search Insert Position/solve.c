int searchInsert(int* nums, int n, int target){
    int l = 0;
    int r = n;
    int p = (l + r) / 2;
    
    while (l <= r && p < n && p >= 0) {
        if (nums[p] == target) return p;
        if (nums[p] > target) r = p - 1;
        if (nums[p] < target) l = p + 1;
        p = (l + r) / 2;
    }
    if (p == n) return n;
    if (r == -1) return 0;
    return p + 1;
}
int search(int* nums, int n, int target){
    int l = 0;
    int r = n;
    int p = floor((l + r) / 2);
    
    while (l <= r && p < n && p >= 0) {
        if (nums[p] == target) return p;
        if (nums[p] > target) r = p - 1;
        if (nums[p] < target) l = p + 1;
        p = floor((l + r) / 2);
    }
    return -1;
}
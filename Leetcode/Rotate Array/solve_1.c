

void rotate(int* nums, int l, int k){
    int *old_nums = (int *)malloc(sizeof(int) * l);
    memcpy(old_nums, nums, sizeof(int) * l);
    int o = 0;
    int n = k % l;
    while (o < l) {
        nums[n] = old_nums[o];
        o += 1;
        n = (n + 1) % l;
    }
}

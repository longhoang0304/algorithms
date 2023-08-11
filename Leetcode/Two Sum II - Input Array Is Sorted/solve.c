

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */

int search(int* nums, int n, int target){
    if (n <= 0) {
        return -1;
    }
    int l = 0;
    int r = n;
    int p = (l + r) / 2;
    
    while (l <= r && p < n && p >= 0) {
        if (nums[p] == target) return p;
        if (nums[p] > target) r = p - 1;
        if (nums[p] < target) l = p + 1;
        p = (l + r) / 2;
    }
    return -1;
}

int* twoSum(int* a, int n, int t, int* returnSize){
    *returnSize = 2;
    int *visited = calloc(sizeof(int), 4000);
    int *r = calloc(sizeof(int), 2);
    int i = 0;
    int j = i + 1;
    for (; i < n; i++) {
        int v = a[i] + 2000;
        if (visited[v]) {
            continue;
        }
        visited[v] = 1;
        int s = t - a[i];
        visited[s + 2000] = 1;
        int k = search(&a[i + 1], n - i - 1, s);
        if (k >= 0) {
            r[0] = i + 1; r[1] = k + i + 2;
            return r;
        }
    }
    return r;
}
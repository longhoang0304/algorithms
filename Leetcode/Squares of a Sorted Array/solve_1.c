

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */

int cmpfunc (const void *a, const void *b) {
   return (*(int*)a - *(int*)b);
}

// O(nlogn) time, O(n) space
int* sortedSquares(int* nums, int numsSize, int* returnSize){
    int i = 0;
    *returnSize = numsSize;
    int *r = malloc(sizeof(int) * numsSize);
    for (; i < numsSize; i++) {
        r[i] = nums[i] * nums[i];
    }
    qsort(r, numsSize, sizeof(int), cmpfunc);
    return r;
    
}

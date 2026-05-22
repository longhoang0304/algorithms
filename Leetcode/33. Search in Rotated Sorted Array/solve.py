class Solution:
    def search(self, nums: List[int], target: int) -> int:
        offset = nums.index(min(nums))
        n = len(nums)
        l, h = 0, n - 1
        while l <= h:
            m = l + (h - l) // 2
            rm = (m + offset) % n
            e = nums[rm]
            if e == target:
                return rm

            if e > target:
                h = m - 1
                continue

            l = m + 1

        return -1

class Solution:
    def twoSumHash(self, nums: list[int], target: int) -> list[int]:
        diff_map = {}
        for i, num in enumerate(nums):
            diff = target - num
            diff_map[diff] = i
            if num in diff_map:
                return diff_map[diff], i
        return -1, -1  # edge case, even though irrelevant
    


    def twoSum(self, nums: list[int], target: int) -> list[int]:
        for i, num in enumerate(nums):
            diff = target - num
            for j in range(i + 1, len(nums)):
                if diff == nums[j]:
                    return i, j
        return -1, -1  # edge case, even though irrelevant 

print(Solution().twoSumHash([3,2,4], 6))
print(Solution().twoSum([3,2,4], 6))

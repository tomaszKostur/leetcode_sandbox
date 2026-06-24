from collections import namedtuple
from typing import List
Bar = namedtuple('Bar', ['index', 'height'])


class Solution:
    # I dont like the brute force
    def maxArea(self, height: List[int]) -> int:
        # all_volumes = []
        # all_permutations = []
        max_volume = 0
        for idx, val in enumerate(height[:-1]):
            bar1 = Bar(idx, val)
            for idx2 in range(idx + 1, len(height)):
                val2 = height[idx2]
                bar2 = Bar(idx2, val2)
                volume = calculate_volume(bar1, bar2) 
                if volume > max_volume:
                    max_volume = volume

                # all_volumes.append(calculate_volume(bar1, bar2))
                # all_permutations.append((bar1.index, bar2.index),)

        # print(f'DEVDEV: {all_volumes}')
        # print(f'DEVDEV: {all_permutations}')
        # return max(all_volumes)
        return max_volume
    
    # INFO: I do not understand this solution, Why we're moving pointers like that?
    # I need to meditate arround it more
    def maxAreaCanonis(self, height: List[int]) -> int:
        left = 0
        right = len(height) - 1
        maxArea = 0

        while left < right:
            currentArea = min(height[left], height[right]) * (right - left)
            maxArea = max(maxArea, currentArea)

            if height[left] < height[right]:
                left += 1
            else:
                right -= 1

        return maxArea


def calculate_volume(bar1: Bar, bar2: Bar):
    y = min(bar1.height, bar2.height)
    x = abs(bar1.index - bar2.index)
    return x*y


def test_container_with_most_water_1():
    height = [1,8,6,2,5,4,8,3,7]
    # print(f'DEVDEV: {len(height)}')
    s = Solution()
    assert s.maxArea(height) == 49


test_container_with_most_water_1()
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function(nums, target) {
    map = {}
    a = nums
    for (let i = 0; i < a.length; i++) {
        const v = a[i];
        if (map[v] === undefined) map[v] = i;
        if (map[target - v] !== undefined && map[target - v] !== i) {
            return [map[target - v], i];
        }
    }
    return [];
};

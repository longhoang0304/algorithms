/**
 * @param {string} digits
 * @return {string[]}
 */

const numpad = {
    '2': ['a', 'b', 'c'],
    '3': ['d', 'e', 'f'],
    '4': ['g', 'h', 'i'],
    '5': ['j', 'k', 'l'],
    '6': ['m', 'n', 'o'],
    '7': ['p', 'q', 'r', 's'],
    '8': ['t', 'u', 'v'],
    '9': ['w', 'x', 'y', 'z'],
}
var letterCombinations = function(digits) {
    if (digits.length < 1) return [];
    let res = [];
    for (let i = 0; i < digits.length; i++) {
        const c = digits[i];
        const s = numpad[c];
        if (res.length < 1) {
            res = res.concat(s);
            continue;
        }
        const newRes = [];
        for(let j = 0; j < res.length; j++) {
            for (let k = 0; k < s.length; k++) {
                newRes.push(res[j] + s[k]);
            }
        }
        res = newRes;
    }
    return res;
};
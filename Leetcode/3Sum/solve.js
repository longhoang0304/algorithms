/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function(n) {
    const res = [];
    n.sort((a, b) => a - b);
    const nl = n.length;
    
    for (let i = 0; i < nl; i++) {
        const target = -n[i];
        let l = i + 1;
        let r = nl - 1;
        
        if (target < 0) {
            break
        }
        
        while (l < r) {
            const s = n[l] + n[r];
            if (s < target) {
                l++;
                continue;
            }
            
            if (s > target) {
                r--;
                continue;
            }

            const rr = [n[i], n[l], n[r]]
            res.push(rr)

            while (l < r && n[l] === rr[1]) l++;
            while (l < r && n[r] === rr[2]) r--;
            
        }

        while (i + 1 < nl && n[i + 1] === n[i]) {
            i++;
        }
    }
    
    return res;
};
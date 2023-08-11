function isPalin(s) {
    let l = s.length;
    if (l === 1) return s;
    if (l === 2 && s[0] === s[1]) return s;
    let i = 0;
    let j = l - 1;
    while(i < l) {
        if (s[i] === s[j]) {
            i += 1;
            j -= 1;
            continue;
        }
        return false;
    }
    return true;
}

function findPalin(s, i, j) {
    // meet the boundary
    if (i === 0 || j === s.length) {
        const ss = s.slice(i, j + 1);
        if (isPalin(ss)) {
            return ss;
        }
        return "";
    }

    while(i > 0 && j < s.length) {
        if (s[i] !== s[j]) {
            return s.slice(i + 1, j);
        }
        
        i -= 1;
        j += 1;
    }
    if (s[i] !== s[j]) {
        i += 1;
        j -= 1;
    }
    return s.slice(i, j + 1);
}

/**
 * @param {string} s
 * @return {string}
 */
var longestPalindrome = function(s) {
    const l = s.length;
    let m = -1;
    let r = "";
    for (let i = 0; i < l;) {
        t = findPalin(s, i, i);
        if (m < t.length) {
            m = t.length;
            r = t;
        }
        
        t = findPalin(s, i, i + 1);
        if (m < t.length) {
            m = t.length;
            r = t;
        }
        i++;
    }
    return r;
};

/**
 * @param {Object|Array} obj
 * @return {Object|Array}
 */
var compactObject = function(obj) {
    const isArray = Array.isArray(obj);
    const ans = isArray ? [] : {};
    for (const k of Object.keys(obj)) {
        if (obj[k] === null) {
            continue;
        } 
        if (typeof obj[k] === 'object') {
            ans[k] = compactObject(obj[k])
        } else if (Boolean(obj[k])) {
            ans[k] = obj[k];
        }
    }
    if (isArray) {
        return ans.filter(v => Boolean(v));
    }
    return ans;
};

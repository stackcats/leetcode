declare global {
    interface Array<T> {
        groupBy(fn: (item: T) => string): Record<string, T[]>
    }
}

Array.prototype.groupBy = function(fn) {
    const gp = {};
    for (const each of this) {
        const key = fn(each);
        if (gp[key]) {
            gp[key].push(each)
        } else {
            gp[key] = [each];
        }
    }
    return gp;
}

/**
 * [1,2,3].groupBy(String) // {"1":[1],"2":[2],"3":[3]}
 */
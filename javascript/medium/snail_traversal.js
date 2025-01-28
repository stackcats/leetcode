/**
 * @param {number} rowsCount
 * @param {number} colsCount
 * @return {Array<Array<number>>}
 */
Array.prototype.snail = function(rowsCount, colsCount) {
    if (this.length != rowsCount * colsCount) {
        return [];
    }

    const mat = Array.from({ length: rowsCount }, () => new Array(colsCount).fill(0));

    let i = 0;
    let j = 0;
    let di = 1;
    for (const n of this) {
        mat[i][j] = n;
        i += di;
        if (i < 0 || i >= rowsCount) {
            j += 1;
            di *= -1;
            i += di;
        }
    }
    return mat;
}

/**
 * const arr = [1,2,3,4];
 * arr.snail(1,4); // [[1,2,3,4]]
 */

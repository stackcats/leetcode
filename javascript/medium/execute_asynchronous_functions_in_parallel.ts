async function promiseAll<T>(functions: (() => Promise<T>)[]): Promise<T[]> {
    const results = new Array(functions.length);
    let completed = 0;
    return new Promise((res, rej) => {
        functions.forEach((f, i) => {
            f().then(val => {
                results[i] = val;
                completed++;
                if (completed == results.length) {
                    res(results);
                }
            }).catch(e => rej(e));
        });
    });
};

/**
 * const promise = promiseAll([() => new Promise(res => res(42))])
 * promise.then(console.log); // [42]
 */

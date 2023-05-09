/**
 * @param {Function} fn
 * @param {number} t
 * @return {Function}
 */
var timeLimit = function (fn, t) {
  const timeoutPromise = new Promise((_res, rej) => {
    setTimeout(() => rej("Time Limit Exceeded"), t);
  });
  return async (...args) => {
    return Promise.race([fn(...args), timeoutPromise]);
  };
};

/**
 * const limited = timeLimit((t) => new Promise(res => setTimeout(res, t)), 100);
 * limited(150).catch(console.log) // "Time Limit Exceeded" at t=100ms
 */

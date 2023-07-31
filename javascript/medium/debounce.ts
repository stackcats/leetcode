type F = (...p: any[]) => any

function debounce(fn: F, t: number): F {
    let myTimeout = null;
    return function(...args) {
        clearTimeout(myTimeout);
        myTimeout = setTimeout(fn, t, ...args);
    }
};

/**
 * const log = debounce(console.log, 100);
 * log('Hello'); // cancelled
 * log('Hello'); // cancelled
 * log('Hello'); // Logged at t=100ms
 */
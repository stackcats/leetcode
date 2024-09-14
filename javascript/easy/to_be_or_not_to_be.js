var expect = function(val) {
     return {
        toBe: (v) => {
            if (v === val) {
                return true;
            }
            throw "Not Equal"
        },
        notToBe: (v) => {
            if (v !== val) {
                return true;
            }
            throw "Equal"
        },
     }
};

class ArrayWrapper {

    nums: number[];
	constructor(nums: number[]) {
        this.nums = nums;
    }

	valueOf() {
        return this.nums.reduce((acc, n) => acc + n, 0);
    }

	toString() {
        return JSON.stringify(this.nums);
    }
};

/**
 * const obj1 = new ArrayWrapper([1,2]);
 * const obj2 = new ArrayWrapper([3,4]);
 * obj1 + obj2; // 10
 * String(obj1); // "[1,2]"
 * String(obj2); // "[3,4]"
 */
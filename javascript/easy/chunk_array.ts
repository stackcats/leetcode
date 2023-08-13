function chunk(arr: any[], size: number): any[][] {
    const ans = [];
    for (let i = 0; i < arr.length; i+=size) {
        ans.push(arr.slice(i, i + size));
    }
    return ans;
};

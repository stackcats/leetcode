class TimeLimitedCache {
  mp = new Map();

  set(key: number, value: number, duration: number): boolean {
    const exist = this.mp.has(key);
    if (exist) {
      clearTimeout(this.mp.get(key)[1]);
    }
    const new_id = setTimeout(() => {
      this.mp.delete(key);
    }, duration);
    this.mp.set(key, [value, new_id]);
    return exist;
  }

  get(key: number): number {
    return this.mp.has(key) ? this.mp.get(key)[0] : -1;
  }

  count(): number {
    return this.mp.size;
  }
}

/**
 * Your TimeLimitedCache object will be instantiated and called as such:
 * var obj = new TimeLimitedCache()
 * obj.set(1, 42, 1000); // false
 * obj.get(1) // 42
 * obj.count() // 1
 */

class FooBar {
private:
  int n;
  atomic_bool b{true};

public:
  FooBar(int _n): n(_n) { }

  void foo(function<void()> printFoo) {
    for (int i = 0; i < n; i++) {
      while (!b.load(memory_order_acquire)) {
        this_thread::yield();
      }
      // printFoo() outputs "foo". Do not change or remove this line.
      printFoo();
      b.store(false, memory_order_release);
    }
  }

  void bar(function<void()> printBar) {
    for (int i = 0; i < n; i++) {
      while (b.load(memory_order_acquire)) {
        this_thread::yield();
      }
      // printBar() outputs "bar". Do not change or remove this line.
      printBar();
      b.store(true, memory_order_release);
    }
  }
};

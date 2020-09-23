class Foo(object):
    def __init__(self):
        self.ct = 1


    def first(self, printFirst):
        """
        :type printFirst: method
        :rtype: void
        """
        
        # printFirst() outputs "first". Do not change or remove this line.
        printFirst()
        self.ct += 1


    def second(self, printSecond):
        """
        :type printSecond: method
        :rtype: void
        """
        while self.ct < 2:
            time.sleep(0.01) 
        # printSecond() outputs "second". Do not change or remove this line.
        printSecond()
        self.ct += 1
            
            
    def third(self, printThird):
        """
        :type printThird: method
        :rtype: void
        """
        while self.ct < 3:
            time.sleep(0.01)
        # printThird() outputs "third". Do not change or remove this line.
        printThird()

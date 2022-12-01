class ProductOfNumbers {
public:
    void add(int num) {
      if (num == 0) arr = {1};
      else arr.push_back(arr.back() * num);
    }
    
    int getProduct(int k) {
      if (k >= arr.size()) return 0;
      return arr.back() / arr[arr.size() - k - 1];
    }
private:
  vector<int> arr = {1};
};

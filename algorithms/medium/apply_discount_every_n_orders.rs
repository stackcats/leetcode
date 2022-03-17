struct Cashier {
    n: i32,
    ct: i32,
    discount: f64,
    prices: Vec<f64>,
}

impl Cashier {

    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let mut prices_ = vec![0.0; 201];
        products.into_iter().zip(prices).for_each(|(i, p)| prices_[i as usize] = p as f64);
        Self {
            n,
            ct: 0,
            discount: discount as f64,
            prices: prices_,
        }
    }
    
    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        let mut bill = product.into_iter()
            .zip(amount)
            .fold(0.0, |acc, (i, ct)| acc + self.prices[i as usize] * (ct as f64));
        
        self.ct += 1;
        
        if self.ct % self.n == 0 {
            bill * (1.0 - self.discount / 100.0)
        } else {
            bill
        }
    }
}

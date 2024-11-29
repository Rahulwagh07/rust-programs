//You want to maximize your profit by choosing a 
//single day to buy one stock and choosing a 
//different day in the future to sell that stock.


fn main(){
  let prices = [7,1,5,3,6,4];
  let mut buy_price = prices[0];
  let mut max_profit = 0;
  let n = prices.len();

  for i in 0..n {
    let profit = prices[i] - buy_price;
    if profit > max_profit {
      max_profit = profit;
    }
    else if prices[i] < buy_price {
      buy_price = prices[i];
    }
  }
  println!("{:?}", max_profit);
}
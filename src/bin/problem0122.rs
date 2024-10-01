/// Time Complexity:  O(N)
/// Space Complexity: O(1)
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut i = 0;
    for j in 0..prices.len() {
        if prices[j] < prices[i] {
            i = j;
        } else if prices[j] > prices[i] {
            let profit = prices[j] - prices[i];
            sum += profit;
            i = j;
        }
    }
    sum
}

fn main() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 8);
}

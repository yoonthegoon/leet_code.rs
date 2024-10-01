/// Time Complexity:  O(N)
/// Space Complexity: O(1)
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut i = 0;
    for j in 0..prices.len() {
        if prices[j] < prices[i] {
            i = j;
        } else {
            let profit = prices[j] - prices[i];
            if profit > max { max = profit; }
        }
    }
    max
}

fn main() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}

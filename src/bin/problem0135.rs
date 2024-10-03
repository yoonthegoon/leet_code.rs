fn candy(ratings: Vec<i32>) -> i32 {
    let n = ratings.len();
    let mut candies = Vec::with_capacity(n);
    candies.push(1);

    for i in 1..n {
        if ratings[i] > ratings[i - 1] {
            candies.push(candies[i - 1] + 1);
        } else {
            candies.push(1);
        }
    }
    let mut acc = candies[n - 1];
    for i in (0..n - 1).rev() {
        if ratings[i] > ratings[i + 1] && candies[i] <= candies[i + 1] {
            candies[i] = candies[i + 1] + 1;
        }
        acc += candies[i];
    }
    acc
}

fn main() {
    assert_eq!(candy(vec![1, 0, 2]), 5);
    assert_eq!(candy(vec![1, 2, 2]), 4);
}

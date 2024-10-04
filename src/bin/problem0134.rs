/// Time Complexity:  O(N)
/// Space Complexity: O(1)
fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let n = gas.len();
    let mut i = 0;
    let mut acc = 0;
    let mut tank = 0;
    for j in 0..n {
        let diff = gas[j] - cost[j];
        acc += diff;
        if acc < 0 {
            i += 1;
            acc = 0;
        }
        tank += diff;
    }
    if tank < 0 { -1 } else { i }
}

fn main() {
    assert_eq!(can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]), 3);
    assert_eq!(can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]), -1);
    assert_eq!(can_complete_circuit(vec![5, 8, 2, 8], vec![6, 5, 6, 6]), 3);
}

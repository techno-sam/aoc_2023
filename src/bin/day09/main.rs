use std::fs;

fn main() {
    println!("AOC 2023 Day 9");

    let contents = fs::read_to_string("src/bin/day09/input.txt").expect("Failed to load input");
    let prediction_sum = predict_sum(&contents, trickle_predict);
    println!("Sum of predictions: {}", prediction_sum);

    let pre_prediction_sum = predict_sum(&contents, trickle_pre_predict);
    println!("Sum of pre-predictions: {}", pre_prediction_sum);
}

fn predict_sum(data: &str, predictor: fn (Vec<Vec<i64>>) -> i64) -> i64 {
    return data.trim().split("\n")
        .map(|l| l.split(" ").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>())
        .map(|v| predictor(diff_stack(v)))
        .sum();
}

/* outputs: includes original history (DOES NOT TRICKLE/predict)
 * 1   3   6  10  15  21
 *   2   3   4   5   6
 *     1   1   1   1
 *       0   0   0
 */
fn diff_stack(history: Vec<i64>) -> Vec<Vec<i64>> {
    let mut stack: Vec<Vec<i64>> = vec![history];
    while stack.last().unwrap().iter().any(|v| *v != 0) {
        let last = stack.last().unwrap();
        let mut next: Vec<i64> = vec![];
        for i in 0..last.len()-1 {
            next.push(last[i+1] - last[i]);
        }
        stack.push(next);
    }

    return stack;
}

fn trickle_predict(stack: Vec<Vec<i64>>) -> i64 {
    let mut last: i64 = 0;
    for i in 1..=stack.len() {
        last += stack[stack.len()-i].last().unwrap();
    }
    return last;
}

fn trickle_pre_predict(stack: Vec<Vec<i64>>) -> i64 {
    let mut last: i64 = 0;
    for i in 1..=stack.len() {
        last = stack[stack.len()-i][0] - last;
    }
    return last;
}

#[test]
fn diff_stacks() {
    assert_eq!(vec![
        vec![1, 3, 6, 10, 15, 21],
        vec![2, 3, 4, 5, 6],
        vec![1, 1, 1, 1],
        vec![0, 0, 0]
    ], diff_stack(vec![1, 3, 6, 10, 15, 21]));
}

#[test]
fn prediction() {
    assert_eq!(28, trickle_predict(diff_stack(vec![1, 3, 6, 10, 15, 21])));
    assert_eq!(68, trickle_predict(diff_stack(vec![10, 13, 16, 21, 30, 45])));
    assert_eq!(5, trickle_pre_predict(diff_stack(vec![10, 13, 16, 21, 30, 45])));
}

#[test]
fn full_pipeline() {
    assert_eq!(114, predict_sum("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
", trickle_predict));
}
